import redis
def add_cmd(r: redis.Redis, key: str, set_name: str, min_interval: int, max_interval: int):
    return r.execute_command(f"iset.add {key} {set_name} {min_interval} {max_interval}")

userInput = input('Please insert number of keys you would like to set');
count = int(userInput)
r = redis.Redis(host='localhost', port=6379)

for i in range(count):
    print(f'Running attempt {i+1}/{count}:')
    #adding key by count
    res = add_cmd(r, f"key{i}", "set1", 10, 25)
    #trigger more sets
    #res = add_cmd(r, "key", "set1", 10, 25)
    print(f'response: {res}')
r.save()
print("Done!~!")