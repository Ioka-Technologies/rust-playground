Feature: Vector alterations with table

  Scenario: Check altering a vector with various initial elements
    Given an initial vector with the following number of elements
      | initial_elements |
      | 0                |
      | 1                |
      | 2                |
      | 3                |
    When I attempt to alter the vector with a given minimum
      | initial_elements | minimum |
      | 0                | 2       |
      | 1                | 0       |
      | 2                | 1       |
      | 3                | 4       |
    Then the vector has the following number of elements
      | initial_elements | expected |
      | 0                | 2        |
      | 1                | 2        |
      | 2                | 3        |
      | 3                | 4        |