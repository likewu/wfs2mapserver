#include "opencvvideo/include/opencv.h"
#include "opencvvideo/examples/example_13-01.rs.h"
#include <algorithm>
#include <functional>
#include <set>
#include <string>
#include <unordered_map>
#include <opencv2/opencv.hpp>
#include <iostream>

using namespace std;

namespace org {
namespace opencv {

class OpencvClient::impl {
  friend OpencvClient;
  using Blob = struct {
    std::string data;
    std::set<std::string> tags;
  };
  std::unordered_map<uint64_t, Blob> blobs;
};

OpencvClient::OpencvClient() : impl(new class OpencvClient::impl) {}

// Upload a new blob and return a blobid that serves as a handle to the blob.
uint64_t OpencvClient::put(MultiBuf &buf) const {
  std::string contents;

  // Traverse the caller's chunk iterator.
  //
  // In reality there might be sophisticated batching of chunks and/or parallel
  // upload implemented by the blobstore's C++ client.
  while (true) {
    auto chunk = next_chunk(buf);
    if (chunk.size() == 0) {
      break;
    }
    contents.append(reinterpret_cast<const char *>(chunk.data()), chunk.size());
  }

  // Insert into map and provide caller the handle.
  auto blobid = std::hash<std::string>{}(contents);
  impl->blobs[blobid] = {std::move(contents), {}};
  return blobid;
}

// Add tag to an existing blob.
void OpencvClient::tag(uint64_t blobid, rust::Str tag) const {
  impl->blobs[blobid].tags.emplace(tag);
}

// Retrieve metadata about a blob.
BlobMetadata OpencvClient::metadata(uint64_t blobid) const {
  BlobMetadata metadata{};
  auto blob = impl->blobs.find(blobid);
  if (blob != impl->blobs.end()) {
    metadata.size = blob->second.data.size();
    std::for_each(blob->second.tags.cbegin(), blob->second.tags.cend(),
                  [&](auto &t) { metadata.tags.emplace_back(t); });
  }
  return metadata;
}

int OpencvClient::maincv(int argc, rust::Vec<rust::Str> argv) {
  /*if(argc != 2) {
    cout << "\n// Example 13-1. Histogram computation and display" << endl;
    cout << "\nComputer Color Histogram\nUsage: " <<argv[0] <<" <imagename>\n" << endl;
    return -1;
  }*/

  cv::Mat src = cv::imread( "E:/app/julia/Learning-OpenCV-3_examples/fruits.jpg",1 );
  if( src.empty() ) { cout << "Cannot load " << argv[1] << endl; return -1; }

  // Compute the HSV image, and decompose it into separate planes.
  //
  cv::Mat hsv;
  cv::cvtColor(src, hsv, cv::COLOR_BGR2HSV);

  float h_ranges[] = {0, 180}; // hue is [0, 180]
  float s_ranges[] = {0, 256};
  const float* ranges[] = {h_ranges, s_ranges};
  int histSize[] = {30, 32}, ch[] = {0, 1};

  cv::Mat hist;

  // Compute the histogram
  //
  cout << "\nhsv matrix dims: " << hsv.dims << " channels: " << hsv.channels() << " depth: " << hsv.depth() << endl;
  cv::calcHist(&hsv, 1, ch, cv::noArray(), hist, 2, histSize, ranges, true);
  cout << "\nhist matrix dims: " << hist.dims << " channels: " << hist.channels() <<  " depth: " << hist.depth() << endl;
  cv::normalize(hist, hist, 0, 255, cv::NORM_MINMAX);

  int scale = 10;
  cv::Mat hist_img(histSize[0]*scale, histSize[1]*scale, CV_8UC3);

  // Draw our histogram.
  //
  for( int h = 0; h < histSize[0]; h++ ) {
    for( int s = 0; s < histSize[1]; s++ ){
      float hval = hist.at<float>(h, s);
      cv::rectangle(
        hist_img,
        cv::Rect(h*scale,s*scale,scale,scale),
        cv::Scalar::all(hval),
        -1
      );
    }
  }

  cv::imshow("image", src);
  cv::imshow("H-S histogram", hist_img);
  cv::waitKey();
  return 0;
}

std::unique_ptr<OpencvClient> new_blobstore_client() {
  return std::make_unique<OpencvClient>();
}

} // namespace openc
} // namespace org
