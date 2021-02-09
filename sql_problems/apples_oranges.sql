-- https://leetcode.com/problems/apples-oranges/
-- SUM(case when fruit='apples' then sold_num else -sold_num end)
select
    sale_date,
    sum( if( fruit='apples', sold_num, -sold_num ) ) as diff
from
    Sales
group by
    sale_date
;
