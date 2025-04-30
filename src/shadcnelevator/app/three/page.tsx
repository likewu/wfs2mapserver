'use client'

import { useState, useEffect, useActionState, useRef, Fragment } from 'react'

import * as THREE from 'three';
import { Canvas, extend, useThree, useFrame } from "@react-three/fiber";
import { useTexture } from '@react-three/drei'

import { Reflect } from './Reflect'

const ThreeComponent = (props: {style: string}) => {
    //const { camera, gl, scene } = useThree();
    const { style, ...rest } = props;
    const renderRef = useRef(null);
    let renderer: any, camera: any, scene: any, controls: any;

    const [count, setCount] = useState(0);
console.log('ccccccc')
    useEffect(() => {
        console.log('aaaaaaaaaa')
        if (renderRef.current) {
          console.log('bbbbbb')
            // 初始化 Three.js
            renderer = new THREE.WebGLRenderer({ antialias: true });
            renderer.setSize(renderRef.current.clientWidth, renderRef.current.clientHeight);
            renderRef.current.appendChild(renderer.domElement);
            console.log('dddddddd')
            // 初始化相机
            camera = new THREE.PerspectiveCamera(75, renderRef.current.clientWidth / renderRef.current.clientHeight, 0.1, 1000);
            camera.position.set(0, 0, 5);

            // 初始化场景
            scene = new THREE.Scene();

            // 添加光源
            const light = new THREE.HemisphereLight(0xffffff, 0x444444);
            light.position.set(0, 200, 0);
            scene.add(light);

            // 添加立方体
            const geometry = new THREE.BoxGeometry();
            const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
            const cube = new THREE.Mesh(geometry, material);
            scene.add(cube);

            // 渲染函数
            const animate = () => {
                requestAnimationFrame(animate);
                cube.rotation.x += 0.01;
                cube.rotation.y += 0.01;
                renderer.render(scene, camera);
            };

            animate();
        }
    }, [/*count*/]);

    return (
    <div>
      <div ref={renderRef} style={{ width: '100%', height: '100%', ...style }} {...rest}></div>
      <button onClick={() => setCount(count + 1)}>Increment</button>
    </div>
    );
}

function MyAnimatedBox() {
  const myMesh = useRef()

  useFrame(({ clock }) => {
    const a = clock.elapsedTime
    //console.log(a)
    myMesh.current.rotation.x = a
  })

  return (
    <mesh ref={myMesh}>
      <boxGeometry args={[2, 2, 2]} />
      <meshBasicMaterial color="royalblue" />
    </mesh>
  )
}

export default function Page() {
    const [company, setCompany] = useState("")

    useEffect(() => {
        console.log('11111111')
    }, []);

    return (
    <div className="flex flex-col items-center justify-items-center min-h-screen p-8 pt-24 gap-12 font-[family-name:var(--font-geist-sans)]">
      <main className="flex-1 flex flex-col items-center justify-items-center gap-12">
        <div className="flex flex-col items-center justify-items-center gap-2">
          <h1 className="text-2xl font-medium">three.js</h1>
          {<Canvas style={{ width: "410px", height: "200px" }}>
            <MyAnimatedBox />
            <ambientLight intensity={0.1} />
            <directionalLight position={[0, 0, 5]} color="red" />
          </Canvas>}
          <Canvas orthographic camera={{ zoom: 100 }} style={{ width: "410px", height: "350px" }}>
            <color attach="background" args={['#000']} />
            <Scene />
          </Canvas>
          {/*<ThreeComponent style="" />*/}
        </div>
      </main>
      <footer className="flex flex-col items-center justify-items-center gap-2">
        <div>
        </div>
      </footer>
    </div>
  );
}

function Scene() {
  const streaks = useRef()
  const glow = useRef()
  const reflect = useRef()
  const [streakTexture, glowTexture] = useTexture(['https://assets.vercel.com/image/upload/contentful/image/e5382hct74si/1LRW0uiGloWqJcY0WOxREA/61737e55cab34a414d746acb9d0a9400/download.png', 'https://assets.vercel.com/image/upload/contentful/image/e5382hct74si/2NKOrPD3iq75po1v0AA6h2/fc0d49ba0917bcbfd3d8a63688045a0c/download.jpeg'])

  const obj = new THREE.Object3D()
  const f = new THREE.Vector3()
  const t = new THREE.Vector3()
  const n = new THREE.Vector3()

  let i = 0
  let range = 0

  useFrame((state) => {
    reflect.current.setRay([(state.pointer.x * state.viewport.width) / 2, (state.pointer.y * state.viewport.height) / 2, 0])
    range = reflect.current.update()

    for (i = 0; i < range - 1; i++) {
      // Position 1
      f.fromArray(reflect.current.positions, i * 3)
      // Position 2
      t.fromArray(reflect.current.positions, i * 3 + 3)
      // Calculate normal
      n.subVectors(t, f).normalize()
      // Calculate mid-point
      obj.position.addVectors(f, t).divideScalar(2)
      // Stretch by using the distance
      obj.scale.set(t.distanceTo(f) * 3, 6, 1)
      // Convert rotation to euler z
      obj.rotation.set(0, 0, Math.atan2(n.y, n.x))
      obj.updateMatrix()
      streaks.current.setMatrixAt(i, obj.matrix)
    }

    streaks.current.count = range - 1
    streaks.current.instanceMatrix.updateRanges.count = (range - 1) * 16
    streaks.current.instanceMatrix.needsUpdate = true

    // First glow isn't shown.
    obj.scale.setScalar(0)
    obj.updateMatrix()
    glow.current.setMatrixAt(0, obj.matrix)

    for (i = 1; i < range; i++) {
      obj.position.fromArray(reflect.current.positions, i * 3)
      obj.scale.setScalar(0.75)
      obj.rotation.set(0, 0, 0)
      obj.updateMatrix()
      glow.current.setMatrixAt(i, obj.matrix)
    }

    glow.current.count = range
    glow.current.instanceMatrix.updateRanges.count = range * 16
    glow.current.instanceMatrix.needsUpdate = true
  })

  return (
    <>
      <Reflect ref={reflect} far={10} bounce={10} start={[10, 5, 0]} end={[0, 0, 0]}>
        {/* Any object in here will receive ray events */}
        <Block scale={0.5} position={[0.25, -0.15, 0]} />
        <Block scale={0.5} position={[-1.1, .9, 0]} rotation={[0, 0, -1]} />
        <Triangle scale={0.4} position={[-1.1, -1.2, 0]} rotation={[Math.PI / 2, Math.PI, 0]} />
      </Reflect>
      {/* Draw stretched pngs to represent the reflect positions. */}
      <instancedMesh ref={streaks} args={[null, null, 100]} instanceMatrix-usage={THREE.DynamicDrawUsage}>
        <planeGeometry />
        <meshBasicMaterial
          map={streakTexture}
          transparent
          opacity={1}
          blending={THREE.AdditiveBlending}
          depthWrite={false}
          toneMapped={false}
        />
      </instancedMesh>
      {/* Draw glowing dots on the contact points. */}
      <instancedMesh ref={glow} args={[null, null, 100]} instanceMatrix-usage={THREE.DynamicDrawUsage}>
        <planeGeometry />
        <meshBasicMaterial
          map={glowTexture}
          transparent
          opacity={1}
          blending={THREE.AdditiveBlending}
          depthWrite={false}
          toneMapped={false}
        />
      </instancedMesh>
    </>
  )
}

function Block({ onRayOver, ...props }) {
  const [hovered, hover] = useState(false)
  return (
    <mesh onRayOver={(e) => hover(true)} onRayOut={(e) => hover(false)} {...props}>
      <boxGeometry />
      <meshBasicMaterial color={hovered ? 'orange' : 'white'} />
    </mesh>
  )
}

function Triangle({ onRayOver, ...props }) {
  const [hovered, hover] = useState(false)
  return (
    <mesh
      {...props}
      onRayOver={(e) => (e.stopPropagation(), hover(true))}
      onRayOut={(e) => hover(false)}
      onRayMove={(e) => null /*console.log(e.direction)*/}>
      <cylinderGeometry args={[1, 1, 1, 3, 1]} />
      <meshBasicMaterial color={hovered ? 'hotpink' : 'white'} />
    </mesh>
  )
}