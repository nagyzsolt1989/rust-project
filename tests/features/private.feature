Feature: Private API

  Scenario: Get Open Orders
    Given The API path is "/0/private/OpenOrders"
    And The API key is set
    When I request open orders
    Then The response status should be 200
    And The response body should contain "open" key