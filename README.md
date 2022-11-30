# rasmus_aoc
advent of code repo

### How to install and run

```
cd day01

cargo install --path .

docker build -t "day01" .

docker run -e part=part1 "day01"

docker run -e part=part2 "day01"
```

Has been testet with these commands before commiting:
```
$ docker build -t "day01" .
$ docker run -e part=part1 "day01" 
$ docker run -e part=part2 "day01"
```