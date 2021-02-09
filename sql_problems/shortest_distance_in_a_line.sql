-- https://leetcode.com/problems/shortest-distance-in-a-line/
-- 一开始我还以为要用MySQL的window函数

-- 解法一
SELECT (p1.x - p2.x) AS shortest
FROM point AS p1, point AS p2
WHERE p1.x > p2.x
ORDER BY shortest
LIMIT 1;

-- 解法二
SELECT MIN(ABS(p1.x - p2.x)) AS shortest
FROM point AS p1, point AS p2
WHERE p1.x != p2.x;

SELECT MIN(ABS(p1.x - p2.x)) AS shortest
FROM point AS p1 JOIN point AS p2
ON p1.x != p2.x;
