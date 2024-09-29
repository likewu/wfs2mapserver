#pragma once

#include <vector>
#include <functional>

#include <opencv2/opencv.hpp>

class ImagePipeline
{
public:
	
	void appendStep(std::function<cv::Mat&(cv::Mat&)>);

	void popStep();

	cv::Mat& process(cv::Mat& img);

	cv::Mat getStepImage(int step);

private:
	std::vector<std::function<cv::Mat&(cv::Mat&)>> steps;
	std::vector<cv::Mat> stepImg;
};