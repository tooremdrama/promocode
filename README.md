# Promocode

Promocode is a API server to deal with promo code.

This tool was made as part of a test to use rust. Promo codes are added via the api. You can find examples in the promocode/tests/promocode-json/ directory.
Then promo code validation requests are sent (you can find them in the integration tests). If the promo code is validated, a response with a percentage is returned. Otherwise, the reasons why the promo code was not validated are indicated.

## Description

You can launch the server application.

You can launch unit tests and integration tests.

You can load testing performance.

You can use swagger at :
```
http://localhost:8080/docs-api
```

## Installation

You need to install rust.

Visit the rust page : https://www.rust-lang.org/fr/tools/install

## Usage

There are two ways to launch server application.

### DEV MODE

This mode uses a fake weather API and lets run test on weather with the fake value :

'is : "Rain" and temp: "28"`

(It would have been possible to parameter it when launching the program but to simplify the launch of the tests planned for this value, we choose this solution.)

To build and launch application :
```
cargo run --bin promocode
```

> [!WARNING]
> - if already launched, will not be launched again,
> - can takes a long time on the first build (depend of the performance of your computer).

### RELEASE MODE

This mode uses weather API with parts that are specified in json promocode. To run this mode, you need a API KEY here [https://api.openweathermap.org]

Some weather tests are disabled with this mode because they can fail. Weather is always changing...

To test release (and it is more optimized).

```
cargo run --release --bin promocode
```

## Documentation

To generate a package documentation.
```
cargo doc -p <PACKAGE-NAME>
```

#### Examples :

To generate promocode crate documentation.

(I just export function that can be used in test to create a util lib. It would be possible to export more functionnalities if needed.)

```
cargo doc -p promocode
```

To generate load-testing documentation.
```
cargo doc -p load-testing
```

To generate load-testing-add-promocode-demo documentation.
```
cargo doc -p load-testing-add-promocode-demo
```

To generate load-testing-is-valid-promocode-demo documentation
```
cargo  doc -p load-testing-is-valid-promocode-demo

```

Path of main page of the package documentation will appear at the end of the compilation.

When server is executed, you can see swagger API documentation : http://localhost:8080/docs-api

## Test

You can launch test in two modes.

DEV MODE allows to test server with a fake API for weather.

RELEASE MODE allows to test server with the weather API in specifications.

> [!WARNING]
> There is some warnings because integration tests share a common module with utils functionnalities. As some functions are not used in some integration test files, a warning appears to prevent us. I prefer to not mask these warnings because they help us to know if we test all we want in some files (example, test all the reasons messages and not forget one, etc...). This can be useful if we add new test in futur.

### DEV MODE

To launch all the tests (units and all integration tests) :

(be sure that server is started just before each test.)
```
cargo test 
```

To see output message for debug, use this command to launch test.

```
cargo test -- --show-output
```

To run only one integration tests file, specify the name of the desired file without extension.

```
cargo test --test <INTEGRATION_TESTS>
```

For example :

To launch only tests in promocode/tests/integration_tests_add_promocode.rs file :

```
cargo test --test integration_tests_add_promocode
```

To launch only tests in promocode/tests/integration_tests_is_valid_promocode.rs file :

```
cargo test --test integration_tests_is_valid_promocode
```


THE FOLLOWING TEST WORKS ONLY WHEN SERVER IS LAUNCHED IN DEV MODE

To launch only tests in promocode/tests/integration_tests_is_valid_promocode_fake_meteo_api.rs file :

```
cargo test --test integration_tests_is_valid_promocode_fake_meteo_api
```

To see output message, add option `-- --show-output`

```
cargo test --test <INTEGRATION_TESTS> -- --show-output
```

#### Examples
```
cargo test --test integration_tests_add_promocode -- --show-output
```

To pass only one specific test in a test file.

```
cargo test --all <SPECIFIED_TEST_NAME_HERE>
```

####  Example :
```
cargo test --all test_add_promocode_example_in_test
```

### RELEASE MODE

You can use the same command that in DEV mode but think to add the option `--release`

Launch server in terminal
```
cargo run --release
```

In another terminal launch tests in release mode
```
cargo test --release
```

## Load testing

Load testing package is a tool that lets you benchmark post API request.

## Roadmap

- Search an available crate to write its own loading test.
- Focused on `drill` and `rlt`.
- Choose `rlt`, I create a very simple benchmark to test load testing on post API request. [https://docs.rs/rlt/latest/rlt/]

## Usage

Remember to launch server before with :
```
cargo run --release --bin promocode
```

### load-testing-add-promocode-demo

Lets testing performance when adding continually new promocode.

Each time a request to add a promocode is done, server checks if a existing promocode with the same name already exist. When you have a lot of promocode in database, this action take more time gradually.

For the demo, a promocode is already configured in the test.

This command lets add new promocode in infinite mode.

```
cargo run --release --bin load-testing-add-promocode-demo

```
This command lets add 10000 new promocode and quit.

```
cargo run --release --bin load-testing-add-promocode-demo -- -n 10000
```

More options are available in the load-testing section. You can use them with the demo load-testing too.


### load-testing-is-valid-promocode-demo 

Lets test performance when checking validity of a promocode.

For the demo, a coupon is already configured in the test.

```
cargo run --release --bin load-testing-is-valid-promocode-demo
```


### load-testing

You can configure load testing by using argument when launching program.

Lets test add-promocode route with a post data.

```
cargo run --release --bin load-testing -- <URL> <JSON>
```

> [!WARNING]
> In the following examples we show how to use this tool with the promocode API.
> Test on is-promocode-valid has Error as response.
> Test on add-promocode has Success as response only for the first request because we can't add a promo code with a name already used.

#### Example :
Only the first request will be OK, then the following will be a Echec. We can't add two promocode with the same name.
``` 
cargo run --release --bin load-testing -- http://localhost:8080/add-promocode "{\"name\": \"Promocode-test_add_promocode_with_weather\",\"advantage\": {\"percent\": 39}, \"restrictions\": [{\"weather\" : {\"is\": \"Clouds\",\"temp\": {\"gt\": -1000}}}]}"
```

#### Examples of command with options :

Infinite mode

```
cargo run --release --bin load-testing -- http://localhost:8080/is-promocode-valid "{\"promocode_name\": \"test_is_valid_promocode_meteo_invalid\", \"arguments\": { \"age\": 20, \"town\": \"lyon\" }}"
```

With a limit of 10000 iterations

```
cargo run --release --bin load-testing -- http://localhost:8080/is-promocode-valid "{\"promocode_name\": \"test_is_valid_promocode_meteo_invalid\", \"arguments\": { \"age\": 20, \"town\": \"lyon\" }}" -n 10000
```

With a limit of 8 workers to run concurrently

```
cargo run --release --bin load-testing -- http://localhost:8080/is-promocode-valid "{\"promocode_name\": \"test_is_valid_promocode_meteo_invalid\", \"arguments\": { \"age\": 20, \"town\": \"lyon\" }}" -c 8
```

You can combine these differents options to configure benchmarck for comparaison.
```
Options:
  -c, --concurrency <CONCURRENCY>
          Number of workers to run concurrently

          [default: 1]

  -n, --iterations <ITERATIONS>
          Number of iterations

          When set, benchmark stops after reaching the number of iterations.

  -d, --duration <DURATION>
          Duration to run the benchmark

          When set, benchmark stops after reaching the duration.

          Examples: -z 10s, -z 5m, -z 1h

  -r, --rate <RATE>
          Rate limit for benchmarking, in iterations per second (ips)

          When set, benchmark will try to run at the specified rate.

  -q, --quiet
          Run benchmark in quiet mode

          Implies --collector silent.
```

## Personal goals
- Test myself of taking a new techno for a real project almost from scratch.
- Use rust for all the project to pratice and learn more.
- Learn a lot about some existing related modules.
- Try to follow convention (I think I can improve my code with more rust usage).
- I would like to improve my skills on more complex project in real industry.
- Practice more english... ^^

## Algorithm

- When a request is received, it is only accepted if route and json format for data are accepted.
- If request is for adding a promo code, syntax and some coherent data are checked before (name not already exists for this promo code, age > 0, data range coherent, weather parameter exists...we don't check grammar). Then the promocode is added in database (status:200) or Error response (status:400) is send.
- If resquest is for validate a coupon, we search a coupon with the specified name and check all the restrictions validity. We send the percentage if Success or the reasons if Echec.

The promocode package contains the following files :
- main.rs : setup endpoints, create and lauch server.
- promocode_api.rs : manage endpoints and response to send.
- promocode_protocol.rs : define response format to send when exhange data.
- promocode.rs : contains schema of data expected in answer request. Contains implementation of Validator trait to check promocode.
- database.rs : contains database and implementation of promocode functionnalities that need concurrency access with some optimization.
- extern_api.rs : contains functionnalities about weather API and fake weather API. Contains units test about weather API.
- lib.rs : contains only authorized functionnalities you can use in integration tests.

## Roadmap

- See documentation of Actix, Rocket and Poem crate to choose which one use to setup the routing server.
- Setting up the routing server.
- Routes tests.
- Creation of README.md to add useful commands as you go.
- Creation of the model according to the logic of the specified json.
- Creation of test for add-promocode route.
- Deconstuction of structure to navigate in json.
- Creation of fake database with mutex and kind of optimized singleton.
- Refactor code to be more scalable with trait (looks like interfaces but not quite the same thing). Validator trait contains two functions. 
```
  is_coherent() -> checks syntax and verify some value like age > 0, coherent date, weather exists, etc...

  is_valid()    -> lets to validate a promo code when a client needs the percentage of a coupon accepted.

  Syntax is checked but not grammar.
```
- Creation of test for is-promocode-valide route.
- Code review.
- Complete README.md.
- Load testing tool creation with [https://docs.rs/rlt/latest/rlt/]
- Test code refactoring.
- Warning in tests are very usefull to check if we test all the case of message (some tests share commun module that contains useful functions for test, but not all tests need it by the way, I prefer not disable them).
- Create more complex json for test (if we do not have a json well formatted, payload send an error so I do not test this kind or json...)
- Use const text to match reasons in test. More scalable if exchange protocol change is made. We see that tests become invalid and we made the change in integration test to match if the new protocol is respected.
- Create useful functions to create more secure tests without typo mistake problem.
- Create a package for each binary in the same repository and prepare demo benchmark to be used quickly.
```
PACKAGE:

promocode -> API server

load-testing -> configuration load testing tool

load-testing-add-promocode-demo -> already configured to load testing the add of mutiple promo code

load-testing-is-valid-promocode-demo -> already configured to load testing the is-valid-promocode request
```
- Test benchmark on more powerfull computer.
- Use of cargo clippy to refactor some part of code.
- [TODO] Refactor code to use closure and bests practices in rust.
- [TODO] Prepare more complex benchmarck according to the specifications.


## Bonus 
- Create a complete lib of promocode for extern use (here I made available only authorized functionnalities that can be use in tests).
- Parameter the choice of weather API to be branch of one for test use. (To simplify the launch of the tests by the corrector, it is hard coded here according to the dev or release mode to be performant).
- [TODO] Check that a promo code is grammatically correct.
- [TODO] Session, Auth...
- [TODO] Propose, to the manager, coupons to be created during the slowest periods of activity to free up time during high demands which could generate additional services.
- [TODO] A manager can prepare coupons. They are then validated by another manager, which makes them effective.
