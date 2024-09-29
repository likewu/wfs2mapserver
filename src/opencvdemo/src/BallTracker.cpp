#include <opencv2/core/core.hpp>
#include <opencv2/highgui/highgui.hpp>
#include <opencv2/imgproc/imgproc.hpp>
#include <opencv2/video/video.hpp>

// Output
#include <iostream>
#include <functional>

#include "ImagePipeline.hpp"
#include "ImageProcessing.hpp"

// Vector
#include <vector>

using namespace std;

#define MIN_RADIUS  1
#define MIN_DISTANCE  50

#define THRESHOLD_BLOCK_SIZE  5
#define THRESHOLD_C       0

cv::Point PointFromMoment(cv::Moments& moment) {
    cv::Point point(moment.m10 / moment.m00, moment.m01 / moment.m00);

    return point;
}

int main(int argc, const char** argv)
{
    std::cout << "Starting program..." << std::endl;

  int lowH = 0;
  int highH = 179;

  int lowS = 0;
  int highS = 255;

  int lowV = 0;
  int highV = 255;

  int threshold = 100;

  int minArea = 1000;

  cv::VideoCapture cam;

  // >>>>> Camera Settings
  //if (!cam.open(0))
  if (!cam.open("E:/app/julia/wfs2map/src/opencvvideo/tests/ball_tracking_example.mp4"))
  {
      cout << "Webcam not connected.\n" << "Please verify\n";
      return EXIT_FAILURE;
  }

  cv::namedWindow("Original", cv::WINDOW_AUTOSIZE);
  cv::namedWindow("Processed", cv::WINDOW_AUTOSIZE);
  cv::namedWindow("Edges", cv::WINDOW_AUTOSIZE);
  cv::namedWindow("Contours", cv::WINDOW_AUTOSIZE);
  cv::namedWindow("Control", cv::WINDOW_AUTOSIZE);

  /*cvCreateTrackbar("LowH", "Control", &lowH, 179);
  cvCreateTrackbar("HighH", "Control", &highH, 179);

  cvCreateTrackbar("LowS", "Control", &lowS, 255);
  cvCreateTrackbar("HighS", "Control", &highS, 255);

  cvCreateTrackbar("LowV", "Control", &lowV, 255);
  cvCreateTrackbar("HighV", "Control", &highV, 255);*/

  cv::createTrackbar("Edge Threshold", "Control", &threshold, 255);

  cv::createTrackbar("Minimum Area", "Control", &minArea, 10000);

  while (1) {
    cv::Mat frame, frameHSV, lowRed, highRed, frameThreshold, frameCanny, frameContours;
    std::vector<std::vector<cv::Point>> contours;
    std::vector<cv::Vec4i> hierarchy;

    std::vector<cv::Point> foundObjects;


    bool success = cam.read(frame);

    if (!success) {
      std::cout<< "Error: Unable to read next frame" << std::endl;
      return -1;
    }

    //Process the image
    cv::cvtColor(frame, frameHSV, cv::COLOR_BGR2HSV);

        //Threshold the image in HSV color space
        cv::inRange(frameHSV, cv::Scalar(0, 135, 0), cv::Scalar(5, 255, 255), lowRed);
    cv::inRange(frameHSV, cv::Scalar(127, 135, 0), cv::Scalar(179, 255, 255), highRed);

    frameThreshold = lowRed + highRed;

        //Get rid of extraneous noise
    cv::erode(frameThreshold, frameThreshold, cv::getStructuringElement(cv::MORPH_ELLIPSE, cv::Size(5, 5)));
    cv::dilate(frameThreshold, frameThreshold, cv::getStructuringElement(cv::MORPH_ELLIPSE, cv::Size(5, 5)));

    cv::dilate(frameThreshold, frameThreshold, cv::getStructuringElement(cv::MORPH_ELLIPSE, cv::Size(5, 5)));
    cv::erode(frameThreshold, frameThreshold, cv::getStructuringElement(cv::MORPH_ELLIPSE, cv::Size(5, 5)));

    cv::Canny(frameThreshold, frameCanny, threshold, 2*threshold);

    cv::findContours(frameCanny, contours, hierarchy, cv::RETR_TREE,
            cv::CHAIN_APPROX_SIMPLE, cv::Point(0, 0));

        frameContours = cv::Mat::zeros(frameCanny.size(), CV_8UC3);

        for(int i = 0; i < contours.size(); i++) {
            cv::Moments moments = cv::moments(contours[i]);


            if(moments.m00 < minArea)
                continue;

            foundObjects.push_back(PointFromMoment(moments));

            cv::drawContours(frameContours, contours, i, cv::Scalar(255, 255, 255),
                             1, 8, hierarchy, 0);
        }

        for(auto &obj : foundObjects) {
            cv::circle(frame, obj, 10, cv::Scalar(255, 255, 255), 1);
        }

    //Display the original and processed images
    cv::imshow("Original", frame);
    cv::imshow("Processed", frameThreshold);
    cv::imshow("Edges", frameCanny);
    cv::imshow("Contours", frameContours);

    if (cv::waitKey(30) == 27) {
      break;
    }
  }

  return 0;
}