package moetokengo

import "testing"

func TestOhayouOniichan(t *testing.T) {
	input := "Akari"
	expectedOutput := "Ohayou, Akari!"
	output := OhayouOniichan(input)

	if output != expectedOutput {
		t.Errorf("Got: %s, expected %s", output, expectedOutput)
	}
}

func TestCountTokens(t *testing.T) {
	input_model := "gpt-3.5-turbo"
	input_text := "Oyasuminasai, Oniichan!"
	expectedOutput := 11
	output := CountTokens(input_model, input_text)

	if output != expectedOutput {
		t.Errorf("Got: %d, expected %d", output, expectedOutput)
	}
}
