Feature: Bookmarks

Scenario: initial sync
    Given I have the server running with the following bookmarks:
        |    | id           |
        | /0 | kMXD6KBOkYFR |
        | /1 | Louf03RhfFx2 |
        | /2 | hoBdEMMzukcg |

    And I already had some bookmarks saved in Firefox that are not in the server:
        |    | id           |
        | /0 | kMXD6KBOkYFR |
        | /1 | J0nHv5_Cj0Pg |
        | /2 | uiQl8E_WtlFw |
        | /3 | zAbmY1UG5qQL |
        | /4 | BldLoaEzD8up |

    When I open Firefox and I have the Bookmarker extention
    Then I expect Firefox to gain the bookmarks from 
    And I expect the server to gain bookmarks it's missing from Firefox:
        |    | id           |
        | /0 | kMXD6KBOkYFR |
        | /1 | J0nHv5_Cj0Pg |
        | /2 | uiQl8E_WtlFw |
        | /3 | zAbmY1UG5qQL |
        | /4 | BldLoaEzD8up |
        | /5 | Louf03RhfFx2 |
        | /6 | hoBdEMMzukcg |
