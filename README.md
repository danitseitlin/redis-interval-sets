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
In the command itself we are able to create 1 or more sets at the same command execution.
Each set **MUST** have a minimum score and a maximum score
```
is.set <key> <set name> <min-score> <max-score> [<set name> <min-score> <max-score>...]
redis> is.set ages highschool 12 18
```

### is.get
```
This command returns existing sets and their min & max scores.
If set name is used, it will retrieve the min & max scores of a specific set.
is.get <key> [set name]
redis> is.get ages
1) 1) "a"
   2) "1"
   3) "5"
2) 1) "b"
   2) "3"
   3) "65"

redis> is.get ages preschool
1) 1) "6"
   2) "11"
```

### is.score
```
This command searches for existing sets that have the given score in their score range.
The returned information is the name of the set **ONLY**
is.score <key> <score>
redis> is.score ages 1
1) "a"
redis> is.score ages 5
1) "a"
2) "b"
```

### is.not_score
```
This command searches for existing sets that don't have the given score in their score range.
The returned information is the name of the set **ONLY**
is.not_score <key> <score>
redis> is.not_score ages 1
1) "b"
redis> is.not_score ages 5
(empty list or set)

```

### is.del
```
This command can delete a key or a specific set. If no <set name> is passed, the whole list of sets (the key itself) will be removed.
To remove a sepecific set, we will pass **at least** one set name. 
is.del <key> [<set name>...]
redis> is.del ages highschool
OK
redis> is.del ages
OK
```

## Build
Make sure you have Rust installed: https://www.rust-lang.org/tools/install

Then, build as usual:
```
cargo build --release
```

## Run
### Linux
```
redis-server --loadmodule ./target/release/libintervalsets.so
```
### Mac OS
```
redis-server --loadmodule ./target/release/libintervalsets.dylib
```