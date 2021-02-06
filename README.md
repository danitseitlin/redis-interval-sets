# RedisIntervalSets
## What are Interval sets?
Interval sets are similar to ZSET command of Redis, but unlike ZSET's it's a range of number per set, an interval.
For example let's say we have a key called ages, that holds free sets of age ranges, pre school, highschool, and college.
We will set it as following:
```
iadd ages preschool 6 11
iadd ages highschool 11 18
iadd ages college 18 50
```
We will result in a key with 3 sets of age ranges.
Now we want to filter out specific set that hold a number in their range, for i.e. ifilter ages 11
Filtering for the value 11, will results in returning 2 available sets: preschool and highschool.

## Commands

### iadd
```
iadd <key> <set name> <min interval> <max interval>
example: iadd ages highschool 12 18
```

### iremove
```
iremove <key> <set name>` <br>
example: iremove <key> <set name>
```
  
### ifilter 
```
ifilter <key> <interval>` <br>
example: ifilter ages 15
```

## Refs:

1. https://github.com/redis/redis/pull/1528
2. https://github.com/redis/redis/pull/2979
3. https://github.com/redis/redis/pull/3272
4. https://github.com/redis/redis/pull/3437

