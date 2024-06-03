Feature: Public API

  Scenario: Validate public API server date and time
    Given The API path is "/0/public/SystemStatus"
    When I send a GET request
    Then The response status should be 200
    And The response body should contain "timestamp" key
    And Server time and current time difference should be within tolerance

  Scenario: Validate public API trading pair
    Given The API path is "/0/public/AssetPairs?pair=XXBTZUSD"
    When I send a GET request
    Then The response status should be 200
    And The response body should contain "XXBTZUSD" key
    And The "XXBTZUSD" node should contain "altname" key with "XBTUSD" value
    And The "XXBTZUSD" node should contain "wsname" key with "XBT/USD" value
    And The "XXBTZUSD" node should contain "aclass_base" key with "currency" value
    And The "XXBTZUSD" node should contain "base" key with "XXBT" value
    And The "XXBTZUSD" node should contain "aclass_quote" key with "currency" value
    And The "XXBTZUSD" node should contain "quote" key with "ZUSD" value
