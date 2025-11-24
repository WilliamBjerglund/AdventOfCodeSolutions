import hashlib


def md5hash():
    """Find the lowest positive number that, when appended to the secret key, produces an MD5 hash starting with five (and 6 for part 2) zeroes."""
    secret_key = "bgvyzdsv"
    n = 1
    while True:
        candidate_number = secret_key + str(n)
        digest = hashlib.md5(candidate_number.encode()).hexdigest()
        if digest.startswith("00000"):  # five zeroes
            print(n)
            break
        else:
            n += 1
    # Part 2 (six zeroes copy paste from above  with minor change)
    while True:
        candidate_number = secret_key + str(n)
        digest = hashlib.md5(candidate_number.encode()).hexdigest()
        if digest.startswith("000000"):  # six zeroes
            print(n)
            break
        else:
            n += 1


md5hash()
