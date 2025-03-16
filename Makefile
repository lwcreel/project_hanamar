all:
	g++ main.cpp map/game_map.cpp entities/player/player.cpp -o main

clean:
	rm main
