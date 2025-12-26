Feature: Color scheme via URL parameter
	As a user
	I want to load a specific color scheme via URL parameter
	So that I can quickly see the app in that color scheme

	Background:
		Given I see the index page with the url params color-scheme=light

	Scenario: Load light color scheme via URL parameter
		Then the app background is light
		When click on the dark button of the color scheme control
		Then the app background is dark
