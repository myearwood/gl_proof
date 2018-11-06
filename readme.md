### GL Proofs

In the deep magic repo, we work on the problem of constructing magic squares of various properites. One of the methods we use to construct these squares is using a greaco latin square. This method ,when given two groups of numbers (for example, in an order five square given [a,b,c,d,e] and [A, B, C, D, E]), constructs a 5x5 magic square.

However, when we are constructing these squares, we would like to simplify this construction a little. Rather than providing 2 groups of 5, we would like to provide an unordered set of 10 integers, and consider all of the different squares that can be generated from this unordered set as a class of magic squares.

From this set of 10 unordered integers, we are able to generate 10! (3,628,800) different groups of 5 integers, which will generate 10! different magic squares(TODO: make a proof for the unique magic squares). By creating these classes magic squares, we hope to develop some intution and theorems, so that we can quickly evaluate the whole class of squares.


Areas of research include:

- Given an unordered set of 10 integers, can we determine out of the whole class, which square is the closest to an add-mult magic square ?

- Given a unordered set of 10 integers, which we know produces magic squares with 0s or duplicate elements does this behavior generalize to the whole class ?

- Can we develop rules to create a set of 10 unordered integers, which we can guarantees that doesn't contain any squares with undesirable traits (0s or duplicate elements.)



### Analyzing the characteristic of a certain class.

    - Are there proportionally worse classes ? or better ones ?
    - Are their ideal positions ?



    ##### Better or Worse Classes ?




    ###### Postiionality

    - 10 square had 400 best candidates, so the position analysis is gonna need a hashset or some heavy duty infra to analyze the best positions.

    - Step 1: See if there are any trends in the best 400 candidates (using the eye test, and my embedded neural net.)

        - No trends to the naked eye, but let's do an analysis of pairs (does a pair of numbers always appear in the same group, or always in different groups) ?





### Analyzing the K formulas

- see if the k formula (use gaps similar to the ones in the most successful squares), and see if we also get some other good squares.


    - I every other variant of the k-formula does not return comparable results. (bummer)
    - here is the k-formula for reference [ k-30, k-23, k, k+30, k+76, k+78, k+80, k+95, k+100, k+114]