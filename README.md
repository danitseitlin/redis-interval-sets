# RedisIntervalSets
## What are Interval sets?
Interval sets are similar to ZSET command of Redis, but unlike ZSET's it's a range of number per set, an interval.
For example let's say we have a key called ages, that holds free sets of age ranges, pre school, highschool, and college.
We will set it as following:
```
redis> is.set ages preschool 6 11
redis> is.set ages highschool 11 18 college 18 50
```
We will result in a key with 3 sets of age ranges.
Now we want to filter out specific set that hold a number in their range, for i.e. ifilter ages 11
Filtering for the value 11, will results in returning 2 available sets: preschool and highschool.

## Commands

### is.set
This command sets a new interval set. An interval set can be extended at any time.
In the command itself we are able to create 1 or more members at the same command execution.
Each member **MUST** have a minimum score and a maximum score
```
is.set <key> <member> <min-score> <max-score> [<member> <min-score> <max-score>...]
redis> is.set ages highschool 12 18
```

### is.get
```
is.get <key> [member]
redis> is.get ages
1) 1) "a"
   2) "1"
   3) "5"
2) 1) "b"
   2) "3"
   3) "65"

redis> is.get preschool
1) 1) "6"
   2) "11"
```

### is.score
```
is.score <key> <interval>
redis> is.score ages 15
```

### is.del
```
is.del <key> <set name> [<set name>...]
redis> is.del ages highschool
```
## Refs:

1. https://github.com/redis/redis/pull/1528
2. https://github.com/redis/redis/pull/2979
3. https://github.com/redis/redis/pull/3272
4. https://github.com/redis/redis/pull/3437
5. https://web.archive.org/web/20171210000653/http://www.starkiller.net/2013/05/03/hacking-redis-adding-interval-sets/

