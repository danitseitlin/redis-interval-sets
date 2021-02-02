# RedisIntervalSets
## What are Interval sets?
Interval sets are similar to ZSET command of Redis, but are more for holding a set of items that are not singular (1 value) but are (x, y) parameters.
For example, today a ZSET usage would be `ZSET ages 20 22 23 25` while ages is the key (let's call it subject of the items inside it) and it basically has a list of ages (20, 22, 23, 25).

## Use case
The main use case is for dots on maps, where we would like to store a list of dots on a polygon, and know when a dot (x, y) is crossing that certain set.
Let's say we have a square, and we want to to check, if our dot interacts with a square in the map.

# Refs:

1. https://github.com/redis/redis/pull/1528
2. https://github.com/redis/redis/pull/2979
3. https://github.com/redis/redis/pull/3272
4. https://github.com/redis/redis/pull/3437

