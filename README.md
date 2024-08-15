# graff

basic program for computing and displaying various functions (R -> R)

## features

- functions (R^n -> R)
  - variables: x_0, x_1, x_2, ...
  - constant function
  - arithmetic: add, sub, neg, mul, div
  - trig: sin, cos, tan
  - e: exp, (natural) log
  - polynomials: scalar coefficients, function coefficients
  - unsized: sum, prod
  - misc: powi
- display
  - simple functions R -> R
- symbolic operations
  - basic algebra simplification rules
  - partial derivative
  - polynomial specific simplifications: mul, pdv
- numerical operations
  - integration rules over finite interval: midpoint, trapezoidal, composite trapezoidal
  - integral inner product
- approximations
  - legendre projection on interval [-1, 1]
  - polynomial with gradient descent on coefficients on interval [-1, 1]
  - neural net

## todo
- topics
  - polar
  - complex
- function approximation
  - polynomial interpolation (lagrange, newton, gradient descent coefficients)
  - taylor series
  - fourier expansion
- numeric computations
  - derivatives
  - more integrals (newton-cotes, simpsons, gaussian, Gauss–Kronrod)
  - solve for zeros (algebraically or newtons method or something)
  - intersections (sort of same as zeros)
- display
  - implicit functions R^2 -> R (marching squares?)
  - parametric R -> R^2
  - 3d? (marching cubes)
  - nd? (marching n-boxes)
- symbolic computation
  - derivative (chain rules etc)
  - integral (Risch algorithm)
  - maybe just make a separate CAS