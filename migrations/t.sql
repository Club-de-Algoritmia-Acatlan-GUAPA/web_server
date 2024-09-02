insert into problem (
        submitted_by, 
        body,
        memory_limit,
        time_limit,
        is_public,
        validation,
        checker

) values 
('99bef03e-8a37-4c6c-af56-ff5131a0c53f', '{ 
  "information": "CSES problem",
  "identifier": "H",
  "name": "Missing Number",
  "input": "The first input line contains an integer $n$.\n\nThe second line contains $n−1$ numbers. Each number is distinct and between 1 and $n$ (inclusive).",
  "output": "Print the missing number.\n\n- $2 \\leq n \\leq 2 \\cdot 10 ^ 5$\n",
  "problem": "You are given all numbers between $1,2,...,n$ except one. Your task is to find the missing number.",
  "note": ""
}', 512, 3, true, 'literal_checker',''),
('99bef03e-8a37-4c6c-af56-ff5131a0c53f', '{ 
  "information": "CSES problem",
  "identifier": "H",
  "name": "Game of chaos",
  "problem": "You are given an array of $n$ integers, and your task is to find two values (at distinct positions) whose sum is $x$.",
  "input": "The first input line has two integers $n$ and $x$: the array size and the target sum.\n\nThe second line has $n$ integers $a_1,a_2, ...,a_n$: the array values.\n\n$$\n1 \\leq x \\leq 10^5\n$$\n* $1 \\leq n \\leq 2 \\cdot 10^5$\n* $1 \\leq x,a_i \\leq  10^9$\n",
  "output": "Print two integers: the positions of the values. If there are several solutions, you may print any of them. If there are no solutions, print `IMPOSSIBLE`."
}', 512, 3, true, 'testlib_checker','#include "testlib.h"
#include <vector>

using namespace std;

const double EPS = 1.5E-5;
//inf ouf ans
int main(int argc, char *argv[]) {
    //setName("compare two sequences of doubles, maximal absolute error = %.10f", EPS);
    registerTestlibCmd(argc, argv);
    int i_n = inf.readInt(1, 2 * (int) 1e5);
    int target = inf.readInt(1, (int) 1e9);
    vector<int>arr;
    for(int idx = 0; idx < i_n; idx++) {
        int k = inf.readInt(1, (int) 1e9);
        arr.push_back(k);
    }
    auto possible = ans.readWord();
    if(possible == "IMPOSSIBLE") {
        auto res = ouf.readWord();
        if(res == "IMPOSSIBLE") quitf(_ok, "");
        else quitf(_wa,"");
    }
    int u_a = ouf.readInt(1,  i_n);
    int u_b = ouf.readInt(1,  i_n);
    if( arr[u_a - 1] + arr[u_b - 1] != target) {
        quitf(_wa,"");
    }
    quitf(_ok, "");
}');


1, 'A problem from CSES', '{"metadata" : { "time_limit":3, "memory_limit":256 }, "name": "Missing Number","input" : "The first input line contains an integer $n$.\n\nThe second line contains $n−1$\n numbers. Each number is distinct and between 1 and $n$ (inclusive).", "output" : "Print the missing number.\n\n- $2 \\leq n \\leq 2 \\cdot 10 ^ 5$\n", "problem" : "You are given all numbers between $1,2,...,n$ except one. Your task is to find the missing number.", "note" : ""}'
),
update problem 
set body = '{"metadata" : { "time_limit":3, "memory_limit":256 }, "name": "Missing Number","input" : "The first input line contains an integer $n$.\n\nThe second line contains $n−1$ numbers. Each number is distinct and between 1 and $n$ (inclusive).", "output" : "Print the missing number.\n\n- $2 \\leq n \\leq 2 \\cdot 10 ^ 5$\n", "problem" : "You are given all numbers between $1,2,...,n$ except one. Your task is to find the missing number.", "note" : ""}'
where problem_id = 1;
