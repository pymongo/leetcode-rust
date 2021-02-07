-- https://leetcode.com/problems/swap-salary/
update users
set sex = if(sex = 'male', 'female', 'male');

-- 方法2，使用case when语句
UPDATE users
SET
    sex = CASE sex
        WHEN 'male' THEN 'female'
        ELSE 'male'
    END;
