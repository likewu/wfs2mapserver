#include "ImageProcessing.hpp"

cv::Mat& ImageProcessing::toGrayscale(cv::Mat& img) {
	cv::cvtColor(img, img, CV_BGR2GRAY);

	return img;
}

cv::Mat& ImageProcessing::blurGaussian(cv::Mat& img, cv::Size kSize, double sigma) {
	cv::GaussianBlur(img, img, kSize, sigma);

	return img;
}

cv::Mat& ImageProcessing::cannyEdge(cv::Mat& img, double thresholdLow, double thresholdRatio, int kernelSize) {
	cv::Canny(img, img, thresholdLow, thresholdLow*thresholdRatio, kernelSize);

	return img;
}

cv::Mat& ImageProcessing::adaptiveThreshold(cv::Mat& img, double maxValue, int method, int thresholdType, int blockSize, double c) {
	cv::adaptiveThreshold(img, img, maxValue, method, thresholdType, blockSize, c);

	return img;
}

cv::Mat& ImageProcessing::erode(cv::Mat& img, int kernelSize) {
	cv::erode(img, img, cv::getStructuringElement(cv::MORPH_ELLIPSE, cv::Size(kernelSize, kernelSize)));

	return img;
}

cv::Mat& ImageProcessing::dilate(cv::Mat& img, int kernelSize) {
	cv::dilate(img, img, cv::getStructuringElement(cv::MORPH_ELLIPSE, cv::Size(kernelSize, kernelSize)));

	return img;
}