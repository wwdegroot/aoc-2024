def generate_permutations(options, length):
  """Generates all unique permutations of a given length using the given options.

  Args:
    options: A list of options to choose from.
    length: The desired length of the permutations.

  Returns:
    A list of all unique permutations.
  """

  if length == 1:
    return options
  else:
    permutations = []
    for option in options:
      for perm in generate_permutations(options, length - 1):
        permutations.append(option + perm)
    return permutations

options = ["+", "*"]
length = 3
perms = generate_permutations(options, length)
print(perms)
options = ["+", "*"]
length = 2
perms = generate_permutations(options, length)
print(perms)
options = ["+", "*"]
length = 1
perms = generate_permutations(options, length)
print(perms)
