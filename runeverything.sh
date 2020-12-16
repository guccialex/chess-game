#run the matchmaking server

cd ./matchmaker

gnome-terminal -- cargo run --release &

cd ..

gnome-terminal -- ./runpythonserver.sh &

cd ./gamefinder_server

gnome-terminal -- ./buildandrun.sh

#and run the client chess checkers game server
#run teh game finder client server




