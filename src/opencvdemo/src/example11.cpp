// Example 15-2. Learning a background model to identify foreground pixels
#include <opencv2/core.hpp>
#include <opencv2/videoio.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <iostream>
#include <cstdlib>
#include <fstream>

using namespace std;

using namespace cv;
using std::cout; using std::cerr; using std::endl;

int main() {
    cv::Mat image = cv::Mat::zeros(300, 300, CV_8UC3);
    //cv::putText(image, "Hello, OpenCV", cv::Point(50, 150), cv::FONT_HERSHEY_SIMPLEX, 1, cv::Scalar(255, 255, 255), 2);
    cv::imshow("Test", image);
    cv::waitKey(0);
    return 0;
}
