// Copyright 2017 Mara, Nate

#include <string>
#include <vector>
#include <cmath>
#include <iostream>

typedef struct Point {
	double x;
	double y;
} Point ;

inline double rand_double() {
	return static_cast<double>(rand()) / static_cast<double>(RAND_MAX);
}

int main(const int argc, const char *argv[]) {
	size_t limit = static_cast<size_t>(atoi(argv[1]));
	std::srand(static_cast<unsigned>(time(0)));

	std::vector<Point> points(limit);

	for (size_t i = 0; i < limit; i++) {
		points[i].x = rand_double();
		points[i].y = rand_double();
	}

	int sum = 0;
	for (size_t i = 0; i < limit; i++) {
		Point &p = points[i];
		if (std::sqrt(p.x * p.x + p.y * p.y) <= 1.0) {
			sum += 1;
		}
	}

	const double pi = static_cast<double>(sum) / limit * 4.0;

	std::cout << pi << std::endl;
}
