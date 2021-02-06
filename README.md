# RedisIntervalSets
## What are Interval sets?
Interval sets are similar to ZSET command of Redis, but unlike ZSET's it's a range of number per set, an interval.
For example let's say we have a key called ages, that holds free sets of age ranges, pre school, highschool, and college.
We will set it as following:
```
redis> iadd ages preschool 6 11
redis> iadd ages highschool 11 18
redis> iadd ages college 18 50
```
We will result in a key with 3 sets of age ranges.
Now we want to filter out specific set that hold a number in their range, for i.e. ifilter ages 11
Filtering for the value 11, will results in returning 2 available sets: preschool and highschool.

## Commands

### iadd
```
iadd <key> <set name> <min interval> <max interval>
redis> iadd ages highschool 12 18
```

### iremove
```
iremove <key> <set name>` <br>
redis> iremove <key> <set name>
```
  
### ifilter 
```
ifilter <key> <interval>` <br>
redis> ifilter ages 15
```

## Refs:

1. https://github.com/redis/redis/pull/1528
2. https://github.com/redis/redis/pull/2979
3. https://github.com/redis/redis/pull/3272
4. https://github.com/redis/redis/pull/3437
5. https://web.archive.org/web/20171210000653/http://www.starkiller.net/2013/05/03/hacking-redis-adding-interval-sets/

