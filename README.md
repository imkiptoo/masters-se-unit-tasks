# Task Sheet 1

## Exercise 1
Write a program that examines a non-empty array with integer values to see whether it can be split into two non-empty parts in such a way that the sum of the values in the first part is equal to the sum of the values in the second part.

### Examples:
- `canDivideArray({1,1,-1,2})` → `false`
- `canDivideArray({21,21,0,1,-1,42})` → `true`
- `canDivideArray({21,-42,21,0})` → `true`
- `canDivideArray({21,-42,0,21})` → `false`

## Task 2
In the game "The Bad Nine", participants sit in a circle and count in turn. Every number divisible by 9 or containing 9 as a digit must be skipped. Write a method that indicates for any number whether it must be skipped or not.

## Task 3
Develop a method that recognizes whether a number is "happy" or not. A number is a "happy number" if repeatedly summing the squares of the individual digits results in 1 after a finite number of steps.

### Examples:
- 49 is happy: `49 → 4^2 + 9^2 = 97 → 9^2 + 7^2 = 130 → 1^2 + 3^2 + 0^2 = 10 → 1^2 + 0^2 = 1`
- 42 is not happy: `42 → 4^2 + 2^2 = 20 → 2^2 + 0^2 = 4 → 4^2 = 16 → 1^2 + 6^2 = 37`

## Task 4
A pangram is a sentence that contains all the letters of the alphabet.

### Example:
- "Every brave Bavarian comfortably eats two pounds of veal knuckles."

Write a program that checks if a given character string is a pangram.

## Task 5 (IBAN Generator)
Since February 2016, we have to use IBAN (International Bank Account Number) for bank transfers. A German IBAN is structured as follows:

The checksum of the German IBAN is calculated according to the following scheme:
1. Form a 24-digit string by concatenating the bank code, account number, and the constant 131400 in that order.
2. Calculate the checksum as the remainder of the division of the 24-digit number by 97.
3. Subtract the calculated remainder from 98.
4. If the result is less than 10, prepend a zero (0).
5. The result is the two-digit checksum.

### Task
Implement a method `generateIBANChecksum` that calculates the IBAN checksum. Then implement a method `generateGermanIBAN` that generates the German IBAN from the account number and bank code.

### Test Case
- Account number: 1234567890, Bank code: 70090100
- Generated IBAN: `DE08700901001234567890`

**Tip:** Use the `BigInteger` class from the standard library as `int` and `long` are too small for the required calculations.

## Task 6 (Place Swap)
We want to write a method that reverses the order of the array entries. The elements should be read from right to left compared to the original positions. The method should be implemented without creating a second array.

### Test Cases:
- `[0, 1, 2, 3, 4, 5, 6]` → `[6, 5, 4, 3, 2, 1, 0]`
- `[0, 1, 4, -3, 4, 9]` → `[9, 4, -3, 4, 1, 0]`

## Task 7 (Minimum Distance)
The minimum distance is defined as follows:
- An array with numbers is passed.
- Determine the two neighboring numbers with the smallest distance.

### Test Cases:
- Array: `[4, 8, 6, 1, 2, 9, 4]`
    - Minimum distance: `1 (= 2 - 1)`
    - Index: `3`
    - Return: `3`

- Array: `[33, 8, 1, 6, 43, 54]`
    - Minimum distance: `5 (= 6 - 1)`
    - Index: `2`
    - Return: `2`

## Task 8 (Minimum Point Distance)
We want to determine the two points with the smallest distance from a two-dimensional array of point coordinates.

### Task
Implement a method `minDistance` that determines two points with the smallest distance from each other.

### Test Case
- Points:
    - P0 (3, 7), P1 (30, 80), P2 (80, 320), P3 (15, 276), P4 (84, 298), P5 (19, 29), P6 (200, 200), P7 (191, 919)
    - Points with smallest distance: P2 and P4

## Task 9 (Strong Password)
Strong passwords have the following properties:
1. Consist of at least 8 characters.
2. Contain at least one lowercase letter.
3. Contain at least one uppercase letter.
4. Contain at least one digit.
5. Contain at least one special character (`!` or `*`).

### Test Cases:
- Strong passwords: `eVJo2!8IrRo`, `aH6*LauTp21u`, `S3cr3ts!`
- Weak passwords: `Password123`, `!2Bcv`, `123456`

## Task 10 (E-mail Check)
We want to write a check method for e-mail addresses based on simplified rules. A valid e-mail address follows the pattern:
`<1 to n characters>@<1 to n characters>.<2 to 3 characters>`

### Test Cases:
- Valid: `john@doe.net`, `john@doe.de`
- Invalid: `john@doe.shop`, `john@.net`, `@.net`

## Task 11 (Encryption)
Implement a simple encryption by shifting the 26 lowercase letters of the alphabet. The characters are shifted cyclically to the right by a given number (the key).

### Task
Implement the `ShiftCipher` class with methods for encryption (`encrypt`) and decryption (`decrypt`). The key is provided when the object is created.

### Test Cases:
- Key: 7
    - Decrypted: `abcdefghijklmnopqrstuvwxyz`
    - Encrypted: `hijklmnopqrstuvwxyzabcdefg`

- Key: 3
    - Decrypted: `thistextisencrypted`
    - Encrypted: `glhvhuwhawlvwyhuvfkoxhvvhow`

Here's Task 12 in markdown format:

## Task 12 (Twitter Wall)
A Twitter wall displays tweets in real-time. Each tweet consists of a user name and text content, which is limited to 140 characters.

### Task
Implement the `Tweet` and `TwitterWall` classes. The `TwitterWall` class should manage a fixed number of tweets. If the limit is reached, the oldest tweet is overwritten.

### Test Case:
```java
TwitterWall tw = new TwitterWall(2);
tw.addTweet(new Tweet("Bot1", "This is a tweet that contains so much text that it will exceed the allowed 140 characters."));
System.out.println(Arrays.toString(tw.getTweets()));
```

Output:
```
Bot1: This is a tweet that contains so much text that it will exceed the allowed 140 characters.
```

### Another Test Case:
```java
TwitterWall tw = new TwitterWall(2);
tw.addTweet(new Tweet("Bot1", "Hello"));
tw.addTweet(new Tweet("Bot2", "Hello too"));
tw.addTweet(new Tweet("Bot3", "How are you?"));
System.out.println(Arrays.toString(tw.getTweets()));
```

Output:
```
Bot2: Hello too
Bot3: How are you?
```
```

This section describes the implementation and behavior of a Twitter wall with a fixed capacity, along with example test cases.
