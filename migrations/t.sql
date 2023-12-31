insert into problem (
        problem_id, information, body
) values 
(1,'A problem from CSES','{"metadata" : { "time_limit":3, "memory_limit":256 }, "name": "Missing Number","input" : "The first input line contains an integer $n$.\n\nThe second line contains $n−1$ numbers. Each number is distinct and between 1 and $n$ (inclusive).", "output" : "Print the missing number.\n\n- $2 \\leq n \\leq 2 \\cdot 10 ^ 5$\n", "problem" : "You are given all numbers between $1,2,...,n$ except one. Your task is to find the missing number.", "note" : ""}'),
(2,'This is a problem from CSES online judge','{ "metadata": { "time_limit" : 2, "memory_limit" : 250 }, "name":"Game of chaos","problem": "You are given an array of $n$ integers, and your task is to find two values (at distinct positions) whose sum is $x$.", "input": "The first input line has two integers $n$ and $x$: the array size and the target sum.\n\nThe second line has $n$ integers $a_1,a_2, ...,a_n$: the array values.\n\n$$\n1 \\leq x \\leq 10^5\n$$\n* $1 \\leq n \\leq 2 \\cdot 10^5$\n* $1 \\leq x,a_i \\leq  10^9$\n", "output": "Print two integers: the positions of the values. If there are several solutions, you may print any of them. If there are no solutions, print `IMPOSSIBLE`." }');

1, 'A problem from CSES', '{"metadata" : { "time_limit":3, "memory_limit":256 }, "name": "Missing Number","input" : "The first input line contains an integer $n$.\n\nThe second line contains $n−1$\n numbers. Each number is distinct and between 1 and $n$ (inclusive).", "output" : "Print the missing number.\n\n- $2 \\leq n \\leq 2 \\cdot 10 ^ 5$\n", "problem" : "You are given all numbers between $1,2,...,n$ except one. Your task is to find the missing number.", "note" : ""}'
),
update problem 
set body = '{"metadata" : { "time_limit":3, "memory_limit":256 }, "name": "Missing Number","input" : "The first input line contains an integer $n$.\n\nThe second line contains $n−1$ numbers. Each number is distinct and between 1 and $n$ (inclusive).", "output" : "Print the missing number.\n\n- $2 \\leq n \\leq 2 \\cdot 10 ^ 5$\n", "problem" : "You are given all numbers between $1,2,...,n$ except one. Your task is to find the missing number.", "note" : ""}'
where problem_id = 1;

"{
        "metadata" : {
        "time_limit":3,
        "memory_limit":256,
        },
        "input" : "lola",
        "output" : "lola",
        "problem" : "lola",
        "note" : "",
}"
'{ "metadata" : { "time_limit":3, "memory_limit":256, }, "input" : "lola", "output" : "lola", "problem" : "lola", "note" : "", }'


insert into problem_serial ( dummy )
values ( true )
returning problem_id;
