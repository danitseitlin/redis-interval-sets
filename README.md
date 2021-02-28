# Redis Interval Sets
## What are Interval sets? :question:
Interval sets are similar to ZSET command of Redis, but unlike ZSET's it's a range of number per set, an interval.
For example let's say we have a key called ages, that holds free sets of age ranges, pre school, highschool, and college.
We will set it as following:
```
redis> iset.add ages preschool 6 11
redis> iset.add ages highschool 11 18 college 18 50
```
We will result in a key with 3 sets of age ranges.
Now we want to filter out specific set that hold a number in their range, for i.e. `iset.score ages 11`
Filtering for the value 11, will results in returning 2 available sets: preschool and highschool.

## Primary features
* Creating interval sets with min and max scores
* Ability to filter out sets by score range

## Quick Start :fast_forward:
```
docker run -p 6379:6379 --name ris danitseitlin/redis-interval-sets:latest
```

## Commands :computer:

### iset.add
This command sets a new interval set. An interval set can be extended at any time.
In the command itself we are able to create 1 or more sets at the same command execution.
Each set **MUST** have a minimum score and a maximum score
```
iset.add <key> <set name> <min-score> <max-score> [<set name> <min-score> <max-score>...]
redis> iset.add ages highschool 12 18
```

### iset.get
```
This command returns existing sets and their min & max scores.
If set name is used, it will retrieve the min & max scores of a specific set.
iset.get <key> [set name]
redis> iset.get ages
1) 1) "a"
   2) "1"
   3) "5"
2) 1) "b"
   2) "3"
   3) "65"

redis> iset.get ages preschool
1) 1) "6"
   2) "11"
```

### iset.score
```
This command searches for existing sets that have the given score in their score range.
The returned information is the name of the set **ONLY**
iset.score <key> <score>
redis> iset.score ages 1
1) "a"
redis> iset.score ages 5
1) "a"
2) "b"
```

### iset.not_score
```
This command searches for existing sets that don't have the given score in their score range.
The returned information is the name of the set **ONLY**
iset.not_score <key> <score>
redis> iset.not_score ages 1
1) "b"
redis> iset.not_score ages 5
(empty list or set)

```

### iset.del
```
This command can delete a key or a specific set. If no <set name> is passed, the whole list of sets (the key itself) will be removed.
To remove a sepecific set, we will pass **at least** one set name. 
iset.del <key> [<set name>...]
redis> iset.del ages highschool
OK
redis> iset.del ages
OK
```

## Build :hammer:
Make sure you have Rust installed: https://www.rust-lang.org/tools/install

Then, build as usual:
```
cargo build --release
```

## Run :running_man:
### Linux
```
redis-server --loadmodule ./target/release/libintervalsets.so
```
### Mac OS
```
redis-server --loadmodule ./target/release/libintervalsets.dylib
```

## Client libraries :books:
Some languages have client libraries that provide support for RedisIntervalSet's commands:

| Project                                    | Language   | License      | Author                                                  | Stars                                               | Package                          |
| ------------------------------------------ | ---------- | ------------ | ------------------------------------------------------- | --------------------------------------------------- | -------------------------------- |
| [redis-modules-sdk][redis-modules-sdk-url] | Typescript | BSD-3-Clause | [Dani Tseiltin @danitseitlin][redis-modules-sdk-author] | [![redis-modules-sdk-stars]][redis-modules-sdk-url] | [npm][redis-modules-sdk-package] |

[redis-modules-sdk-author]: https://github.com/danitseitlin/
[redis-modules-sdk-url]: https://github.com/danitseitlin/redis-modules-sdk
[redis-modules-sdk-package]: https://www.npmjs.com/package/redis-modules-sdk
[redis-modules-sdk-stars]: https://img.shields.io/github/stars/danitseitlin/redis-modules-sdk.svg?style=social&amp;label=Star