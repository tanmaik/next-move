package main

import (
	"encoding/csv"
	"fmt"
	"os"
	"strconv"
	"time"

	"github.com/eiannone/keyboard"
	"github.com/go-vgo/robotgo"
)

type Action struct {
	Name       string
	Coordinates []Coordinate
}

type Coordinate struct {
	X, Y int
}

func main() {
	actions := loadActions("cursor_positions.csv")
	selectedAction := selectAction(actions)
	performAction(selectedAction)
}

func loadActions(filename string) []Action {
	file, err := os.Open(filename)
	if err != nil {
		fmt.Println("Error opening file:", err)
		os.Exit(1)
	}
	defer file.Close()

	reader := csv.NewReader(file)
	records, err := reader.ReadAll()
	if err != nil {
		fmt.Println("Error reading CSV:", err)
		os.Exit(1)
	}

	actions := make(map[string]*Action)
	for i, record := range records {
		if i == 0 { // Skip header
			continue
		}
		name := record[0]
		x, _ := strconv.Atoi(record[1])
		y, _ := strconv.Atoi(record[2])

		if action, exists := actions[name]; exists {
			action.Coordinates = append(action.Coordinates, Coordinate{X: x, Y: y})
		} else {
			actions[name] = &Action{
				Name:       name,
				Coordinates: []Coordinate{{X: x, Y: y}},
			}
		}
	}

	result := make([]Action, 0, len(actions))
	for _, action := range actions {
		result = append(result, *action)
	}
	return result
}

func selectAction(actions []Action) Action {
	fmt.Println("Select an action using arrow keys and press Enter to confirm:")
	selected := 0

	if err := keyboard.Open(); err != nil {
		panic(err)
	}
	defer keyboard.Close()

	printActions(actions, selected)

	for {
		_, key, err := keyboard.GetKey()
		if err != nil {
			panic(err)
		}

		switch key {
		case keyboard.KeyArrowUp:
			selected = (selected - 1 + len(actions)) % len(actions)
		case keyboard.KeyArrowDown:
			selected = (selected + 1) % len(actions)
		case keyboard.KeyEnter:
			return actions[selected]
		}

		printActions(actions, selected)
	}
}

func printActions(actions []Action, selected int) {
	fmt.Print("\033[2J")  // Clear screen
	fmt.Print("\033[H")   // Move cursor to top-left corner
	for i, action := range actions {
		if i == selected {
			fmt.Print("> ")
		} else {
			fmt.Print("  ")
		}
		fmt.Println(action.Name)
	}
}

func performAction(action Action) {
	fmt.Printf("Performing action: %s\n", action.Name)
	for _, coord := range action.Coordinates {
		robotgo.MoveMouse(coord.X, coord.Y)
		time.Sleep(10 * time.Millisecond)
	}
	fmt.Println("Action completed.")
}