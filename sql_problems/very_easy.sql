-- https://leetcode.com/problems/employees-earning-more-than-their-managers/
select e.Name as Employee
from Employee as e
         inner join Employee as m
                    on e.ManagerId = m.Id
where e.Salary > m.Salary;

-- https://leetcode.com/problems/not-boring-movies/
select id,movie,description,rating from cinema where id%2=1 and description <> 'boring' order by rating desc;
