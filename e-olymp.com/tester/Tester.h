/*
 * Tester.h
 *
 *  Created on: 05.10.2011
 *      Author: finter
 */

#ifndef TESTER_H_
#define TESTER_H_

enum TesterCode {
	OK,
	RuntimeError
};

class Tester {
public:
	Tester(char *executible_filename, int timelimit=5000);
	TesterCode run();
	virtual ~Tester();

protected:
	virtual void makeInput() = 0;
	virtual void chekAnswer() = 0;

private:
	char *executible_filename;
	int timelimit; // in milliseconds
};

#endif /* TESTER_H_ */
