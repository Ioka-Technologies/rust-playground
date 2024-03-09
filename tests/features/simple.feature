Feature: Vector alterations without table

  Scenario: Check altering a vector works as expected
    Given an initial vector with 3 elements
    When I attempt to alter the vector and specify 5 minimum elements
    Then the vector has 5 elements and has been altered the expected amount of times