End to end tests for the slime seed finder web demo.

Usage:

If testing the local version, first in one terminal run:

```
./ci/build_demo.sh
cd static
python3 server.py
```

And in another one run:

```
cd selenium-e2e-tests
export SLIME_SEED_FINDER_DEMO_URL=http://127.0.0.1:8000
export SELENIUM_BROWSER=firefox
npm run test
```

If testing the deployed version (<https://badel2.github.io/slime_seed_finder/>), you only need to run:

```
cd selenium-e2e-tests
export SELENIUM_BROWSER=firefox
npm run test
```

To run only one test, use the grep param and pass a regex:

```
npm run test -- -g '.*spawner.*'
```
