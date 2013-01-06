/*
 * Tester.cpp
 *
 *  Created on: 05.10.2011
 *      Author: finter
 */

#include "Tester.h"

TesterCode Tester::run()
{
	makeInput();


	return OK;
}

Tester::Tester(char *executible_filename, int timelimit)
{
	this->executible_filename = executible_filename;
	this->timelimit = timelimit;
}

Tester::~Tester() {}

