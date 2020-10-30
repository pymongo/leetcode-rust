-- https://leetcode.com/problems/employees-earning-more-than-their-managers/
select e.Name as Employee
from Employee as e
         inner join Employee as m
                    on e.ManagerId = m.Id
where e.Salary > m.Salary;