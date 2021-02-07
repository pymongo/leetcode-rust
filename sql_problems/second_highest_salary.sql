-- https://leetcode.com/problems/second-highest-salary/
-- 本题的难点在于如果不存在排第二的工资则返回NULL，所以要包了一层IFNULL
SELECT IFNULL(
    (
        SELECT DISTINCT salary
        FROM employee
        ORDER BY salary DESC
        LIMIT 1 OFFSET 1
    ),
    NULL
) AS SecondHighestSalary
