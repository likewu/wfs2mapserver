#pragma once

#include <opencv2/opencv.hpp>

namespace ImageProcessing
{
	cv::Mat& toGrayscale(cv::Mat&);

	cv::Mat& blurGaussian(cv::Mat&, cv::Size kSize, double sigma);

	cv::Mat& cannyEdge(cv::Mat&, double thresholdLow, double thresholdRatio, int kernelSize);

	cv::Mat& adaptiveThreshold(cv::Mat&, double maxValue, int method, int thresholdType, int blockSize, double c);

	cv::Mat& erode(cv::Mat&, int kernelSize);

	cv::Mat& dilate(cv::Mat&, int kernelSize);
}