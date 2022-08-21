/*
 * This file contains completely bad code by Syntactic's default rules
 */

#include <stdio.h>
#include <doesnt_exist.h>
#include <also_doesnt_exist.c>

int main(){
	int x = 1;
	if (x == 1) { return 0; }
	else {
		printf("x is %d\n", x);
	}

	/* Single line comment using multi-line comments */
	/* A Bad
	 * Multi Line Comment
	 */
	return 0;
}
