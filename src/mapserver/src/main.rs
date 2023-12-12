use std::sync::Arc;

use mapserver::coordinates::Tile;
use mapserver::mappool::MapPool;
use mapserver::Extent;

use axum::extract::Path;
use axum::http::header;
use axum::Extension;
use tokio::sync::Mutex;

use axum_swagger_ui::swagger_ui;

use async_trait::async_trait;
use axum::{
    body::{Body},
    //extract::{extractor_middleware, FromRequest, RequestParts},
    routing::{get, post},
    http::{StatusCode,Request,Uri},
    response::{Html,IntoResponse,Redirect},
    routing::Route,
    Router,
};

use axum::middleware;
use axum_jwks::Jwks;
use mapserver::authjwks;

pub fn make_mapfile_str(timestamp: i64) -> String {
    format!(
        "MAP
          NAME 'default'
          STATUS ON
          PROJECTION
            'init=epsg:3857'
          END
          EXTENT 73.498999 20.207173812500002 135.08835446875 53.561666
          UNITS DD
          DEBUG 5
          CONFIG 'CPL_DEBUG' 'ON'
          CONFIG 'CPL_TIMESTAMP' 'ON'
          CONFIG 'CPL_LOG' '/dev/stderr'
          CONFIG 'CPL_LOG_ERRORS' 'ON'
          CONFIG 'MS_ERRORFILE' '/dev/stderr'
          CONFIG 'GDAL_DISABLE_READDIR_ON_OPEN' 'TRUE'
          CONFIG 'GDAL_FORCE_CACHING' 'NO'
          CONFIG 'GDAL_CACHEMAX' '10%'
          CONFIG 'VSI_CACHE' 'FALSE'
          CONFIG 'VSI_CACHE_SIZE' '0'  # bytes
          CONFIG 'CPL_VSIL_CURL_CACHE_SIZE' '0'  # bytes
          SIZE 256 256
          IMAGECOLOR 255 255 255
          IMAGETYPE 'png'
          SHAPEPATH '/tmp'
          LAYER
            NAME'continents'
            TYPE POLYGON
            STATUS ON
            CONNECTION 'https://demo.mapserver.org/cgi-bin/wfs?'
            CONNECTIONTYPE WFS
            METADATA
              'wfs_typename'          'continents'
              'wfs_version'           '1.0.0'
              'wfs_connectiontimeout' '60'
              'wfs_maxfeatures'       '10'
            END
            PROJECTION
              'init=epsg:4326'
              #{}
            END
            CLASS
              NAME 'Continents'
              STYLE
                COLOR 255 128 128
                OUTLINECOLOR 96 96 96
              END
            END
          END
        END",
        timestamp
    )
}

#[derive(Debug)]
struct State {
    maplock: Mutex<MapPool>,
}

#[tokio::main]
async fn main() {
    // Set up shared state
    let map_pool = MapPool::create(24);
    let shared_state = Arc::new(State {
        maplock: Mutex::new(map_pool),
    });


    let jwks = Jwks::from_oidc_url(
        // The Authorization Server that signs the JWTs you want to consume.
        "https://my-auth-server.example.com/.well-known/openid-configuration",
        // The audience identifier for the application. This ensures that
        // JWTs are intended for this application.
        "https://my-api-identifier.example.com/".to_owned(),
    )
    .await
    .unwrap();
    let state_jwks = authjwks::AppState { jwks };


    fn api_routes() -> Router {
        Router::new()
            .route("/users", get(api_users))
            .route("/job", post(api_job))
            //.layer(extractor_middleware::<RequireAuth>())
            //.route_layer(middleware::from_fn_with_state(
            //    state_jwks.clone(),
            //    authjwks::validate_token,
            //))
            //.with_state(state_jwks);
    }

    // Routes
    let doc_url = "swagger/openapi.yaml";
    let app = Router::new()
        .route("/", get(index))
        .route("/login", get(login))
        .route("/map/:timestamp/:z/:x/:y", get(render_map))
        .route("/swagger", get(|| async { swagger_ui(doc_url) }))
        .route(doc_url, get(|| async { include_str!("../swagger/openapi.yaml") }))
        .nest("/api", api_routes())
        .layer(Extension(shared_state));

    // Spawn the web handler
    tokio::spawn(async move {
        println!("Listening on 0.0.0.0:3000");
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    });

    // And wait for an interupt signal
    tokio::signal::ctrl_c().await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

async fn render_map(
    Path((timestamp, z, x, y)): Path<(i64, u32, u32, u32)>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    // Create mapfile
    let tile = Tile::from_zxy(z, x, y);
    let extent = Extent::from(tile.bbox_mercator());
    let mapfile_str = make_mapfile_str(timestamp);

    // Get a renderer from the map pool
    /*let renderer = {
        let mut map_pool = state.maplock.lock().await;
        map_pool.acquire_or_create(mapfile_str)
    };*/

    // Yes, we can render concurrently on multiple threads!
    // GDAL may lock things internally though, negating much of the benefit
    //let image_bytes = renderer.render(extent);

    let mut map_pool = state.maplock.lock().await;
    let url = "http://example.com".parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return Ok(());
    }
    let image_bytes = map_pool.fetch_url(url).await;

    //([(header::CONTENT_TYPE, "image/png")], image_bytes)
    ([(header::CONTENT_TYPE, "text/text")], image_bytes.unwrap())
}


// An extractor that performs authorization.
struct RequireAuth;

/*#[async_trait]
impl<B> FromRequest<B> for RequireAuth
where
    B: Send,
{
    type Rejection = Redirect; //StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let auth_header = req
            .headers()
            .and_then(|headers| headers.get(axum::http::header::AUTHORIZATION))
            .and_then(|value| value.to_str().ok());

        if let Some(value) = auth_header {
            if value == "secret" {
                return Ok(Self);
            }
        }

        //Err(StatusCode::UNAUTHORIZED)
        Err(Redirect::to(Uri::from_static("/login")))
    }
}*/

async fn login() -> impl IntoResponse {
    Html("<h1>Login Page</h1>")
}

async fn api_users() ->impl IntoResponse  {
    "api_users"
}

async fn api_job() -> impl IntoResponse  {
    "job".to_string()
}
