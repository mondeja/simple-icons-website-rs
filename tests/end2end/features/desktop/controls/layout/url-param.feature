Feature: Layout via URL parameter
	As a user
	I want to load a specific layout via URL parameter
	So that I can quickly see the app in that layout

	Background:
		Given I see the index page with the url params layout=compact

	Scenario: Load compact layout via URL parameter
		Then the selected layout is compact
		And the compact number of icons per page have been loaded
		When click on the comfortable button of the layout control
		Then the selected layout is comfortable
