Feature: Search for icons using URL parameter
	Search for icons using the search control via URL parameter.

	Background:
		Given I see the index page with the url params q=rust

	Scenario: Search for an icon given its name via URL parameter
		Then I see the icon "Rust" first
