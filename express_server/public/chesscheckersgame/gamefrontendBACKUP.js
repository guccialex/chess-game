
import init, { JavaScriptInterface, FullGame } from './wasmfiles/wasm_builder.js';



let websocketaddress = 'ws://localhost:3012';




run();




async function run() {
    await init();
    
    
    let canvas = document.getElementById("renderCanvas"); // Get the canvas element
    let engine = new BABYLON.Engine(canvas, true); // Generate the BABYLON 3D engine
    
    
    
    let game = new GameInterface(engine);
    
    /*
    
    // This creates and positions a free camera (non-mesh)
    var camera = new BABYLON.FreeCamera("camera1", new BABYLON.Vector3(0, 5, -10), scene);
    
    // This targets the camera to scene origin
    camera.setTarget(BABYLON.Vector3.Zero());
    
    // This attaches the camera to the canvas
    camera.attachControl(canvas, true);
    
    // This creates a light, aiming 0,1,0 - to the sky (non-mesh)
    var light = new BABYLON.HemisphericLight("light1", new BABYLON.Vector3(0, 1, 0), scene);
    
    // Default intensity is 1. Let's dim the light a small amount
    light.intensity = 0.7;
    
    // Our built-in 'sphere' shape. Params: name, subdivs, size, scene
    var sphere = BABYLON.Mesh.CreateSphere("sphere1", 16, 2, scene);
    
    // GUI
    var advancedTexture = BABYLON.GUI.AdvancedDynamicTexture.CreateFullscreenUI("UI");
    
    var input = new BABYLON.GUI.InputText();
    input.width = 0.2;
    input.maxWidth = 0.2;
    input.height = "40px";
    input.text = "This is a very long text used to test how the cursor works within the InputText control.";
    input.color = "white";
    input.background = "green";
    advancedTexture.addControl(input);   
    */
    
    
    
    
    //get the engine of the game
    engine.runRenderLoop(function () {
        
        game.tick();
        
    });
    
    
    
    /*
    //create a  websocket with the server
    const socket = new WebSocket( websocketaddress );
    
    
    //when connected to the game, this is the information provided
    let connectedgameinformation = null;
    
    
    //HALT UNTIL THE CONNECTION OPENS
    socket.addEventListener('open', function (event) {
        socket.send('Hello Server!');
    });
    
    
    let listenforconnection = function (event) {
        
        console.log("GOT MESSAGE");
        
        
        
    };
    
    
    socket.addEventListener('message', listenforconnection);
    
    */
    
    
    //loop until its connected to a game, and then start that game
    
    
    
    //console.log("IM HERE");
    
    //run the main program
    //main();
    
    
    
}



function main(){
    
    
    
    // And afterwards we can use all the functionality defined in wasm.
    let thegame = new Game();
    
    
    
    //websocket stuff
    const socket = new WebSocket( websocketaddress );    
    
    
    //wait until the connection opens
    
    //HALT UNTIL THE CONNECTION OPENS
    socket.addEventListener('open', function (event) {
        socket.send('Hello Server!');
    });
    
    
    //when a message is received
    //give it directly to the game as a "game state update"
    //it comes from my server so i should assume its fine right?
    
    
    // Listen for messages
    socket.addEventListener('message', function (event) {
        
        thegame.game.receive_game_state_data(event.data);
        
        console.log('Message from server ', event.data);
        
    });
    
    
    
    
    
    
    //get the engine of the game
    thegame.engine.runRenderLoop(function () {
        
        thegame.tick();
        thegame.render();
    });
    
    
    //
    window.addEventListener("click", function () {
        // We try to pick an object
        var pickResult = thegame.scene.pick(thegame.scene.pointerX, thegame.scene.pointerY);
        
        //if an object was picked, tell the game
        if (pickResult.hit == true){
            thegame.meshclicked(pickResult.pickedMesh);
        }
        
    });
    
    
    
    
    
    
    
    
}




//this class is called when the player creates a new game
class GameInterface{
    
    
    
    
    //get an engine
    constructor(engine){
        
        
        //create a scene for this
        let scene = new BABYLON.Scene(engine);
        
        // This creates and positions a free camera (non-mesh)
        var camera = new BABYLON.FreeCamera("camera1", new BABYLON.Vector3(0, 5, -10), scene);
        
        // This targets the camera to scene origin
        camera.setTarget(BABYLON.Vector3.Zero());        
        
        //get the canvas for this engine to attach a control tos
        let canvas = engine.getRenderingCanvas();
        camera.attachControl(canvas, true);
        
        // This creates a light, aiming 0,1,0 - to the sky (non-mesh)
        var light = new BABYLON.HemisphericLight("light1", new BABYLON.Vector3(0, 1, 0), scene);
        
        this.scene = scene;
        
        
        
        var advancedTexture = BABYLON.GUI.AdvancedDynamicTexture.CreateFullscreenUI("UI");
        
        var input = new BABYLON.GUI.InputText();
        input.width = 0.2;
        input.maxWidth = 0.2;
        input.height = "40px";
        input.text = "This is a very long text used to test how the cursor works within the InputText control.";
        input.color = "white";
        input.background = "green";
        advancedTexture.addControl(input);    
        
        
        
        
        //connect to the server
        this.socket = new WebSocket( websocketaddress );
        
        //the wasm game
        this.wasmgame = FullGame.new();
        
        //if the game is currently in the mode of "ingame" or "requesting game" 
        this.ingame = false;
        
        
        
        
    }
    
    
    
    //get a websocket message from the server
    get_message(message){
        
        //give the received message to the game
        this.wasmgame.set_incoming_sockets_message( message.data );
        
    }
    
    //send the queued websocket message to the server
    send_queued_message(){
        

        //if the game has a message queued to send
        if (this.wasmgame.is_outgoing_socket_message_queued() ){
            
            
            //pop a message from the wasmgame
            let message = this.wasmgame.pop_outgoing_socket_message();
            console.log(message);
            console.log(this.socket);
            
            //this socket is being considered "no longer usable"
            this.socket.send( message );
            
            
            
        }
        
        
    }
    
    //render the scene
    render(){
        
        //if the state is not currently in game
        if (this.ingame == false){
            
            //get from the game whether that has changed or not, if its been put into a tick
            
            
        }
        
        
        
        this.scene.render();
        
    }
    
    
    tick() {
        
        //the websocket calls this one
        //this.get_message();
        this.send_queued_message();
        
        
        
        this.render();
        
        
    }
    
    
}





class Game{
    
    /*
    what methods does the game need?
    
    Render the scene
    
    tick the game state
    
    update the state of the game to match the state of the game in the server
    (this is called asynchronously by the websocket)
    
    give input (to this game, and to the server)
    
    
    
    move_cursor_on_scene(position)
    (the cursor is moved here on the scene)
    (parse what object it is over and highlight it)
    
    click_scene(position)
    (the scene is clicked at this position)
    (set the object that was clicked, and set the state of this interface with that)
    (or set the input)
    
    */
    
    
    
    tick(){
        //tick the state of the game
        this.game.tick();
        
        
        
        
        
    }
    
    
    //render the scene 
    render() {
        
        //get the id of every piece in the scene
        let pieceids = this.game.get_piece_ids();
        
        //get the id of every board square in the scene
        let boardsquareids = this.game.get_board_square_ids();
        
        //get the id of every card in the scene
        let cardids = this.game.get_card_ids();
        
        
        /*
        console.log(pieceids);
        console.log(boardsquareids);
        console.log(cardids);
        */
        
        
        
        //for each piece
        for (let pieceid of pieceids){
            
            
            //get the name of the mesh of the piece
            let piecename = PieceIDtoName(pieceid);
            
            
            //get the mesh if it exists
            let piecemesh = this.scene.getMeshByName(piecename);
            
            
            //if the mesh doesnt exist, create it first
            if (piecemesh == null){
                
                piecemesh = BABYLON.MeshBuilder.CreateCylinder (piecename, {height: 0.5, diameter: 0.7, tesselation: 3}, this.scene);
                
                piecemesh.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
                piecemesh.material.diffuseColor = BABYLON.Color3.White();
                
                console.log("its a new object after all");
            }
            
            
            let piecedata = this.game.get_piece_data(pieceid);
            
            
            //set the position of the piece
            
            piecemesh.position.x = piecedata.xposition;
            piecemesh.position.y = piecedata.yposition;
            piecemesh.position.z = piecedata.zposition;
            
            piecemesh.rotation.x = piecedata.xrotation;
            piecemesh.rotation.y = piecedata.yrotation;
            piecemesh.rotation.z = piecedata.zrotation;
            
            
            
        }
        
        
        
        //for each boardsquare id
        //console.log(boardsquareids);
        
        //for each piece
        for (let boardsquareid of boardsquareids){
            
            //get the name of the mesh of the piece
            let boardsquarename = BoardSquareIDtoName(boardsquareid);
            
            //get the mesh if it exists
            let objectmesh = this.scene.getMeshByName(boardsquarename);
            
            
            //if the mesh doesnt exist, create it first
            if (objectmesh == null){
                
                
                objectmesh = BABYLON.MeshBuilder.CreateBox(boardsquarename, {height: 1, width: 1, depth: 1 }, this.scene);
                
                
                if ((boardsquareid[0] + boardsquareid[1]) %2 == 0){   
                    objectmesh.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
                    objectmesh.material.diffuseColor = BABYLON.Color3.White();
                }
                else{
                    objectmesh.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
                    objectmesh.material.diffuseColor = BABYLON.Color3.Gray();
                }
                
                
                
                
                console.log("its a new object after all");
            }
            
            
            
            let boardsquaredata = this.game.get_board_square_data(boardsquareid[0], boardsquareid[1]);
            
            //set the position of the piece
            
            objectmesh.position.x = boardsquaredata.xposition;
            objectmesh.position.y = boardsquaredata.yposition;
            objectmesh.position.z = boardsquaredata.zposition;
            
            objectmesh.rotation.x = boardsquaredata.xrotation;
            objectmesh.rotation.y = boardsquaredata.yrotation;
            objectmesh.rotation.z = boardsquaredata.zrotation;
            
            
            
        }
        
        
        
        //highlight the mesh that the pointer is over
        {
            /*
            var pickResult = this.scene.pick(this.scene.pointerX, this.scene.pointerY);
            
            //if the pointer is over a mesh
            if (pickResult.hit == true){
                
                //if this is a different mesh than the one already highlighted
                if (pickResult.pickedMesh != this.highlightedmesh){
                    
                    
                    
                    //if the highlighted mesh is not null
                    if (this.highlightedmesh != null){
                        
                        //set the colour of the highlighted mesh back to what it originally was
                        this.highlightedmesh.material.diffuseColor = this.highlightedmeshoriginalcolour;
                        
                    }
                    
                    //set the highlighted mesh and its original colour to what the newly highlighted mesh's is
                    this.highlightedmesh = pickResult.pickedMesh;
                    this.highlightedmeshoriginalcolour = this.highlightedmesh.material.diffuseColor;
                    
                    //set the colour of the newly highlighted mesh to red
                    this.highlightedmesh.material.diffuseColor = BABYLON.Color3.Red();
                    
                }
                
                
            }
            //if its not over a mesh
            else{
                
                //and an object is highlighted, unhighlight it
                if (this.highlightedmesh != null){
                    
                    this.highlightedmesh.material.diffuseColor = this.highlightedmeshoriginalcolour;
                    
                }
                
                
            }
            
            //if the mesh that is currently highlighted is different than the mesh that was previously highlighted
            
            
            
            */   
        }
        
        
        
        
        
        //get the information about every object in the scene
        
        //get a json of the appearance of the structs
        
        //this.game.get_appearance_data();
        
        
        
        
        
        //render it
        this.scene.render();
        
        
        
    }
    
    
    
    
    
    
    
    
    meshclicked(pickedmesh){
        
        var pickResult = this.scene.pick(this.scene.pointerX, this.scene.pointerY);
        
        
        {
            /*
            //if the pointer is over a mesh
            if (pickResult.hit == true){
                
                //if this is a different mesh than the one already selected
                if (pickResult.pickedMesh != this.selectedmesh){
                    
                    
                    //if the highlighted mesh is not null
                    if (this.selectedmesh != null){
                        
                        //set the colour of the highlighted mesh back to what it originally was
                        this.selectedmesh.material.diffuseColor = this.selectedmeshoriginalcolour;
                    }
                    
                    //set the highlighted mesh and its original colour to what the newly highlighted mesh's is
                    this.selectedmesh = pickResult.pickedMesh;
                    this.selectedmeshoriginalcolour = this.selectedmesh.material.diffuseColor;
                    
                    //set the colour of the newly highlighted mesh to red
                    this.selectedmesh.material.diffuseColor = BABYLON.Color3.Yellow();
                    
                    
                    
                }
                //if its not over a mesh
                else{
                    
                    //and an object is highlighted, unhighlight it
                    if (this.selectedmesh != null){
                        
                        this.selectedmesh.material.diffuseColor = this.selectedmeshoriginalcolour;
                        
                    }
                    
                    
                }
                
                //if the mesh that is currently highlighted is different than the mesh that was previously highlighted
                
            }
            */
        }
        
        
        //set the colour of every mesh to random
        for(var mesh of this.scene.meshes) {
            
            mesh.material.diffuseColor = BABYLON.Color3.Random();
            
        }
        
        
        //console.log (this.selectedmesh.name );
        let pickedMesh = pickResult.pickedMesh;
        
        
        //if its a piece
        //make it the selected piece and get the allowed actions for that piece
        if (pickedMesh.name[0] == "A"){
            
            this.selectedmesh = pickedMesh;
            
            let pieceid = MeshNametoID( this.selectedmesh.name );
            
            let allowedactions = this.game.piece_allowed_actions(  pieceid  );
            
            console.log(allowedactions);
            
            this.selectedmeshallowedactions = allowedactions;
            
            //console.log( this.selectedmeshallowedactions.size() );
            
            
            //iterate through the allowed actions
            
            for (let x = 0; x < this.selectedmeshallowedactions.size()  ; x++) {
                
                let action = this.selectedmeshallowedactions.get_action_by_number(x);
                
                let boardsquarex = action.get_board_squarex();
                let boardsquarez = action.get_board_squarez();
                
                let boardsquarename = BoardSquareIDtoName(  [boardsquarex, boardsquarez] );
                let mesh = this.scene.getMeshByName(boardsquarename);
                mesh.material.diffuseColor = BABYLON.Color3.Green();
                
                
                //console.log(boardsquarex, boardsquarez);
                
            }
            
        }
        
        
        
        //if its a board square clicked
        //if a piece is already selected
        //see if that board square is one of the ones allowed
        //if it is, send that as an input to the game
        if (pickedMesh.name[0] == "B"){
            
            //if the mesh thats been selected is a piece
            if (this.selectedmesh.name[0] == "A" ){ 
                
                let selectedpiecename = MeshNametoID( this.selectedmesh.name );
                
                let boardsquareid = MeshNametoID(  pickedMesh.name );
                
                //get if this is one of the board squares allowed to be moved to by the selected piece
                let actionindex = this.selectedmeshallowedactions.is_this_square_allowed( boardsquareid[0], boardsquareid[1] );
                
                //if theres an action that lets the piece move to the square clicked on
                //send it to the game as an input
                if (actionindex != null){
                    
                    let action = this.selectedmeshallowedactions.get_action_by_number(actionindex);
                    
                    this.game.set_piece_action(selectedpiecename, action);
                    
                    console.log("sent an input to the engine");
                    
                }
                
                
            }
            
            
        }
        
        
        
    }
    
    
    
    
    //every tick, get the id of every card
    //of every piece
    //of every board square
    
    //the position of all of these things
    //all the information that is needed
    
    //and update all these objects meshes in accordance with that
    
    constructor(){
        
        
        
        let canvas = document.getElementById("renderCanvas"); // Get the canvas element
        let engine = new BABYLON.Engine(canvas, true); // Generate the BABYLON 3D engine
        
        
        // Create the scene space
        let scene = new BABYLON.Scene(engine);
        
        scene.ambientColor = new BABYLON.Color3(0.1, 0.9, 0.1);
        
        // Add a camera to the scene and attach it to the canvas
        let camera = new BABYLON.ArcRotateCamera("Camera", Math.PI / 2, Math.PI / 2, 2, new BABYLON.Vector3(0,0,0), scene);
        camera.attachControl(canvas, true);
        
        // Add lights to the scene
        let light1 = new BABYLON.HemisphericLight("light1", new BABYLON.Vector3(1, 1, 0), scene);
        let light2 = new BABYLON.PointLight("light2", new BABYLON.Vector3(0, 1, -1), scene);
        
        light1.intensity = 1.00;
        
        
        this.engine = engine;
        this.scene = scene; //Call the createScene function
        this.camera = camera;
        this.light1 = light1;
        this.light2 = light2;
        
        
        
        
        let objectmesh = BABYLON.MeshBuilder.CreateBox("TESTING", {height: 3, width: 0.4, depth: 0.4 }, this.scene);
        
        objectmesh.position.x = 0.0;
        objectmesh.position.y = 0.0;
        objectmesh.position.z = 0.0;
        
        objectmesh.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
        objectmesh.material.diffuseColor = BABYLON.Color3.Green();
        
        
        
        
        
        let objectmesh3 = BABYLON.MeshBuilder.CreateBox("TESTING", {height: 3, width: 0.4, depth: 0.4 }, this.scene);
        
        objectmesh3.position.x = -4.0;
        objectmesh3.position.y = 0.0;
        objectmesh3.position.z = -4.0;
        
        objectmesh3.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
        objectmesh3.material.diffuseColor = BABYLON.Color3.Green();
        
        
        let objectmesh4 = BABYLON.MeshBuilder.CreateBox("TESTING", {height: 0.4, width: 0.4, depth: 2.1 }, this.scene);
        
        objectmesh4.position.x = 0.0;
        objectmesh4.position.y = 2.0;
        objectmesh4.position.z = 0.0;
        
        objectmesh4.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
        objectmesh4.material.diffuseColor = BABYLON.Color3.Green();
        
        
        
        
        // Watch for browser/canvas resize events
        window.addEventListener("resize", function () {
            engine.resize();
        });
        
        
        
        
        
        
        
        
        
        
        //mesh that is highlighted
        this.highlightedmesh = null;
        
        //the original colour of the mesh that is highlighted
        this.highlightedmeshoriginalcolour = null;
        
        
        //object that is selected
        this.selectedmesh = null;
        
        //the actions allowed by the selected mesh
        this.selectedmeshallowedactions = null;
        
        
        this.selectedmeshoriginalcolour = null;
        
        
        //these are variables for both determining what input to send at the next "click" function
        //as well as the appearance of the game state
        this.game = JavaScriptInterface.new();
        
        
        
        
        
    }
    
    
}





//given a piece id, get the name of the mesh
function PieceIDtoName(pieceid) {
    
    return ("A"+pieceid.toString(10));
    
}


function BoardSquareIDtoName(boardsquareid){
    
    //append with B
    let fullstring = "B"
    
    fullstring += boardsquareid[0].toString();
    fullstring += boardsquareid[1].toString();
    
    return (fullstring)
    
    
}


//given the name of a mesh, get the id of it
function MeshNametoID(meshname){
    
    //get rid of the 
    
    if (meshname[0] == "A"){
        
        return( parseInt( meshname.substring(1) ) )
        
        
    }
    if (meshname[0] == "B"){
        
        
        return ( [ parseInt (meshname[1]),  parseInt( meshname[2]) ] )
        
    }
    
    
}