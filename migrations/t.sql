insert into problem (
        problem_id, information, body
) values (
1, 'A problem from CSES', '{"metadata" : { "time_limit":3, "memory_limit":256 }, "name": "Missing Number","input" : "The first input line contains an integer $n$.\n\nThe second line contains $n−1$\n numbers. Each number is distinct and between 1 and $n$ (inclusive).", "output" : "Print the missing number.\n\n- $2 \\leq n \\leq 2 \\cdot 10 ^ 5$\n", "problem" : "You are given all numbers between $1,2,...,n$ except one. Your task is to find the missing number.", "note" : ""}'
);

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
