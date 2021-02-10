-- https://leetcode.com/problems/employees-earning-more-than-their-managers/
select e.Name as Employee
from Employee as e
         inner join Employee as m
                    on e.ManagerId = m.Id
where e.Salary > m.Salary;

-- https://leetcode.com/problems/not-boring-movies/
select id,movie,description,rating
from cinema
where id%2=1 and description <> 'boring'
order by rating desc;

-- https://leetcode.com/problems/invalid-tweets/
select tweet_id from Tweets where length(content) > 15;

-- https://leetcode.com/problems/find-total-time-spent-by-each-employee/
select
    event_day as day,
    emp_id,
    sum(out_time-in_time) as total_time
from
    Employees
group by
    event_day,
    emp_id
;

-- https://leetcode.com/problems/find-the-team-size/
-- select e1.employee_id, count(*) as team_size from Employee e1, Employee e2 where e1.team_id = e2.team_id group by e1.employee_id;
-- select employee_id, count(*) over(partition by team_id) as team_size from employee
select
    a.employee_id as employee_id,
    b.team_size as team_size
from
    Employee as a
left join
    (
        select
            team_id,
            count(employee_id) as team_size
        from
            Employee
        group by
            team_id
    ) as b
on
    a.team_id = b.team_id
;

-- https://leetcode.com/problems/average-selling-price/
select
    p.product_id,
    round(sum(p.price * u.units) / sum(u.units), 2) as average_price
from
    Prices as p,
    UnitsSold as u
where
    p.product_id = u.product_id
    and u.purchase_date between start_date and end_date
group by
    p.product_id
;

-- https://leetcode.com/problems/product-sales-analysis-i/
-- select product_name, year, price from Sales left join Product using (product_id)
-- join on 后面的条件已经把结果过滤了一遍，而where是笛卡尔积后才根据限制条件进行过滤，所以join性能要比where好
select p.product_name, s.year, s.price
from Sales as s, Product as p
where s.product_id = p.product_id;

-- https://leetcode.com/problems/product-sales-analysis-ii/
select product_id, sum(quantity) as total_quantity from Sales group by product_id;

-- https://leetcode.com/problems/triangle-judgement/
-- SELECT x,y,z,(CASE WHEN x+y>z AND x+z>y AND z+y>x THEN 'Yes' ELSE 'No' END) as triangle FROM triangle;
select
    x, y, z,
    -- 两最小边之和大于最长边，则能构成三角形
    if( x+y+z-greatest(x,y,z)>greatest(x,y,z), 'Yes', 'No' ) as triangle
from triangle;
