
//when matched up to be put in a game


import init, { FullGame } from './wasmfiles/wasm_builder.js';



//these are gotten from outside or the calling function


let websocketaddress = 'ws://localhost:3012';

//whether this is player 1 or 2
let playerid = 1;

//the password for the game that needs to be given to the server
let gamepassword = "somepassword";




run();




async function run() {
    
    await init();
    
    
    //create a websocket connection with the server
    let socket = new WebSocket( websocketaddress );
    
    
    
    socket.onopen = function (event) {
        
        //when connected, send a message with the password
        socket.send( gamepassword );
        
    };
    
    
    
    socket.onmessage = function (event) {
        
        console.log("connected to game");
        
        //if its a message that im connected to the game
        if (event.data == "connected to game"){
            
            
            //remove the "onmessage "event listener
            socket.onmessage = null;
            
            
            //start the game and give it the socket connection with the server
            start(socket);
            
            
        }
        
        
    };
    
    
    
    
    
    
    
    
}





async function start(socket){
    
    
    let canvas = document.getElementById("renderCanvas"); // Get the canvas element
    let engine = new BABYLON.Engine(canvas, true); // Generate the BABYLON 3D engine
    
    let mygame = new GameInterface(engine, socket);
    
    console.log("started");
    
    
    
    
    
    
    
    //create an event listener that when a message is received, it is sent to the game
    mygame.socket.onmessage = function (event) {
        
        mygame.get_message(event.data);
        
    };
    
    
    
    //run the game
    rungame(mygame);
    
}







async function rungame(thegame) {
    
    console.log("STARtING GAME");
    
    
    
    //add an event listener for the mouse going up
    window.addEventListener("click", function () {
        
        thegame.mouseup();
        
    });
    
    
    //add an event for themouse going down
    window.addEventListener("pointerdown", function () {
        
        thegame.mousedown();
        
    });
    
    //add an event for themouse moving
    window.addEventListener("pointermove", function () {
        
        thegame.mousemove();
        
    });
    
    
    //run the tick function of the game 30 times per second
    thegame.gameappearance.engine.runRenderLoop(function () {
        
        thegame.tick();
        
    });
    
    
    
    
    
    
}





































//the appearance of the game state
//doesnt this also manage getting input?
class GameApperance{
    
    constructor(engine, gameinterface){
        
        //create a scene for the engine
        let scene = new BABYLON.Scene(engine);
        
        this.engine = engine;
        
        
        // This creates and positions a free camera (non-mesh)
        let camera = new BABYLON.ArcRotateCamera("camera1", 0, 0, 0, new BABYLON.Vector3(0.0,2.0,0.0), scene);
        
        //set the position of the camera, not its target tho
        camera.setPosition(new BABYLON.Vector3(0, 15, -7));
        
        camera.lowerBetaLimit = 0.1;
        camera.upperBetaLimit = (Math.PI / 2) * 1.0;
        
        camera.lowerRadiusLimit = 10;
        camera.upperRadiusLimit = 30;
        
        
        
        
        //get the canvas for this engine to attach a control tos
        let canvas = engine.getRenderingCanvas();
        
        
        camera.attachControl(canvas, true);
        camera.inputs.attached["mousewheel"].wheelPrecision = 10;
        camera.inputs.attached.keyboard.detachControl();
        
        
        // This creates a light, aiming 0,1,0 - to the sky (non-mesh)
        var light = new BABYLON.HemisphericLight("light1", new BABYLON.Vector3(0, 1, 0), scene);
        
        
        
        
        this.advancedTexture = BABYLON.GUI.AdvancedDynamicTexture.CreateFullscreenUI("UI");
        
        
        this.thegameinterface = gameinterface;
        
        this.scene = scene;
        
        this.camera = camera;
        
        
        
        //create the plane
        let mesh = BABYLON.MeshBuilder.CreateBox("plane", {height: 0.008, width: 100.98, depth: 100.08 }, this.scene);
        mesh.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
        mesh.material.alpha = 0.05;
        mesh.material.diffuseColor = BABYLON.Color3.Gray();
        mesh.position.y = 1.1;
        
        
        
    }
    
    
    
    //render the scene using the appearance data
    render(appearancedata){
        
        //the list of objects passed in to be rendered
        let objectspassedtorender = [];
        
        
        //for each object in the appearance data
        for (let objectdata of appearancedata.objects){
            
            //console.log(objectdata);
            
            
            //get the name of the object
            let objectname = objectdata.objectname;
            
            //get the mesh if it exists
            let objectmesh = this.scene.getMeshByName(objectname);
            
            for (const mesh of this.scene.meshes) {
                if (mesh.name === objectname) {
                    
                    objectmesh = mesh;
                }
            }
            
            
            //if the mesh doesnt exist, create it
            if (objectmesh == null){
                
                if (objectdata.appearanceid == 10){
                    
                    objectmesh = BABYLON.MeshBuilder.CreateCylinder(objectname, {height: 0.5, diameter: 0.7 }, this.scene);
                    
                    //using imported mesh
                    /*
                    
                    let tempscene = this.scene;
                    
                    BABYLON.SceneLoader.ImportMesh("", "./", "classic_king_nosupport.stl", this.scene, function (meshes) {
                        
                        meshes[0].scaling.x = 0.025;
                        meshes[0].scaling.y = 0.025;
                        meshes[0].scaling.z = 0.025;
                        
                        
                        meshes[0].material = new BABYLON.StandardMaterial("bs_mat", tempscene);
                        meshes[0].material.diffuseColor = BABYLON.Color3.Green();
                        
                        
                        //get the old object with the old object name
                        //get rid of it
                        tempscene.getMeshByName(objectname).dispose();
                        
                        
                        
                        //and set this as the new object with that name
                        meshes[0].name = objectname;
                        
                        
                    });
                    
                    */
                    
                    
                    
                }
                else if (objectdata.appearanceid == 20 || objectdata.appearanceid == 21){
                    
                    objectmesh = BABYLON.MeshBuilder.CreateBox(objectname, {height: 0.999, width: 0.999, depth: 1.00 }, this.scene);
                    
                }
                //50 is a pool queue
                else if (objectdata.appearanceid == 50){
                    
                    objectmesh = BABYLON.MeshBuilder.CreateBox(objectname, {height: 0.20, width: 0.20, depth: 1.98 }, this.scene);
                    
                }
                //100 to 200 is cards
                else if (objectdata.appearanceid  >= 100 && objectdata.appearanceid  <= 200 ){

                    //ratio of 2.5 to 3.5 for width and depth for cards
                    
                    objectmesh = BABYLON.MeshBuilder.CreateBox(objectname, {height: 0.2, width: 1.6, depth: 2.24 }, this.scene);
                    
                    
                }
                //201 is a flat board for the cards
                else if (objectdata.appearanceid  == 201){

                    console.log("got the board object");

                    objectmesh = BABYLON.MeshBuilder.CreateBox(objectname, {height: 0.5, width: 8.0, depth: 8.0 }, this.scene);

                }
                else{
                    
                    console.log("the other objectcolour seems to be" + objectdata.appearanceid );
                    
                    objectmesh = BABYLON.MeshBuilder.CreateBox(objectname, {height: 0.95, width: 0.95, depth: 0.95 }, this.scene);
                    
                }
                
                
                objectmesh.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
                objectmesh.material.diffuseColor = BABYLON.Color3.Gray();
                
                
                console.log("its a new object after all");
            }
            
            
            //if it selected, set its colour to yellow
            if ( objectdata.isselected == true ){
                objectmesh.material.diffuseColor = BABYLON.Color3.Yellow();
            }
            //if its highlighted set its colour to green
            else if ( objectdata.ishighlighted == true ){
                objectmesh.material.diffuseColor = BABYLON.Color3.Green();
            }
            //otherwise set its colour to its default colour
            else{
                
                if (objectdata.appearanceid == 10){
                    objectmesh.material.diffuseColor = BABYLON.Color3.Gray();
                }
                else if (objectdata.appearanceid == 20){
                    objectmesh.material.diffuseColor = BABYLON.Color3.White();
                }
                else if (objectdata.appearanceid == 21){
                    objectmesh.material.diffuseColor = BABYLON.Color3.Black();
                }
                else if (objectdata.appearanceid == 50){
                    objectmesh.material.diffuseColor = BABYLON.Color3.Gray();
                }
                else{
                    objectmesh.material.diffuseColor = BABYLON.Color3.Green();
                }
                
                
                
            }
            
            
            //set its position and rotation values
            objectmesh.position.x = objectdata.xposition;
            objectmesh.position.y = objectdata.yposition;
            objectmesh.position.z = objectdata.zposition;
            
            objectmesh.rotation.x = objectdata.xrotation;
            objectmesh.rotation.y = objectdata.yrotation;
            objectmesh.rotation.z = objectdata.zrotation;
            
            
            
            
            objectspassedtorender.push(objectname);
            
            
            
        }
        
        
        //and each object that wasn't passed in for this tick, remove it from the list of meshes
        //if its name also isnt "plane"
        for (let mesh of this.scene.meshes) {
            
            //if the objects passed to render includes the current mesh
            if (objectspassedtorender.includes(mesh.name)) {
                //do nothing            
            }
            //otherwise remove it
            else{
                
                //IF it is not also named "plane"
                if (mesh.name != "plane"){
                    mesh.dispose();
                }
                
                
            }
            
        }
        
        
        
        
        
        this.scene.render();
        
        
        
        
    }
    
}






//this class is called when the player creates a new game
class GameInterface{
    
    
    
    constructor(engine, socket){
        
        //create the "appearance" object for this game, giving it the scene of the engine
        this.gameappearance = new GameApperance(engine, this);
        
        this.socket = socket;
        
        //create the wasm game
        this.wasmgame = FullGame.new(1);
        
        
        
        //if an object is being dragged (if the camera movement is disabled)
        this.draggingobject = false;
        
        //what the position of the pointer is on the y=1.5 plane when i start dragging
        this.draggingstartingposition = null;
        
        
    }
    
    
    
    //get a websocket message from the server
    get_message(message){
        
        console.log("receiving a message from the server", message);
        
        //give the received message to the game
        this.wasmgame.get_incoming_socket_message( message );
    }
    
    
    //render the scene
    render(){
        
        //get appearance data and send it to the GameAppearance object to render
        let appearancedata = this.wasmgame.get_appearance_data();
        this.gameappearance.render(appearancedata);
        
    }
    
    
    tick() {
        
        
        //tick the internal game
        this.wasmgame.tick();
        
        //render it
        this.render();
        
        
        //get if any outgoing message is queued to be sent
        if (this.wasmgame.is_outgoing_socket_message_queued() ){
            
            console.log("im sending a websocket message");
            
            //and send them to the server
            this.socket.send( this.wasmgame.pop_outgoing_socket_message() );
        }
        
        
    }
    
    
    
    //when a player clicks
    mouseup(){
        
        
        //reenable the cameras ability to move
        this.gameappearance.camera.inputs.attached["mousewheel"].wheelPrecision = 10;
        this.gameappearance.camera.inputs.attached["pointers"].angularSensibilityX = 1000;
        this.gameappearance.camera.inputs.attached["pointers"].angularSensibilityY = 1000;
        
        
        //not dragging any object after the mouse is lifted
        this.draggingobject = false;
        
        //tell the wasm that its mouse up
        //so it can send the flick missions if any piece is in the middle of being flicked
        this.wasmgame.mouse_up();
        
    }
    
    
    
    //when the mouse is moved
    mousemove(){
        
        //if a piece is currently being dragged, send that information to the wasmgame
        if (this.draggingobject){

            

            let selectedobjectname = this.wasmgame.get_selected_object_name();

            var objectunder = this.gameappearance.scene.pick(this.gameappearance.scene.pointerX, this.gameappearance.scene.pointerY, function(mesh) {
        
                return mesh.name != "plane" && mesh.name != "dragindicator" && mesh.name != selectedobjectname;  // the plane and drag indicator will not be pickable
            
            });
            
            
            
            //set the position of the cursor on the plane
            var pickResult = this.gameappearance.scene.pick(this.gameappearance.scene.pointerX, this.gameappearance.scene.pointerY, function(mesh) {
                return mesh.name == "plane";  // the plane will be the only pickable thing
            });
            
            let draggingcurposition = [pickResult.pickedPoint.x, pickResult.pickedPoint.z];
            
            
            let distancedraggedx = draggingcurposition[0] - this.draggingstartingposition[0];
            let distancedraggedz = draggingcurposition[1] - this.draggingstartingposition[1];
            

            if (objectunder.pickedMesh ==  null){

                this.wasmgame.drag_selected_object(distancedraggedx, distancedraggedz, "");

            }
            else{

                this.wasmgame.drag_selected_object(distancedraggedx, distancedraggedz, objectunder.pickedMesh.name);

            }
            
            
        }
        
        
        
    }
    
    
    //when the mouse goes down
    mousedown(){
        
        var pickResult = this.gameappearance.scene.pick(this.gameappearance.scene.pointerX, this.gameappearance.scene.pointerY, function(mesh) {
            
            //let toreturn 
            
            return mesh.name != "plane" && mesh.name != "dragindicator";  // the plane and drag indicator will not be pickable
        });
        
        
        
        //if a mesh has been clicked
        let clickedobject = pickResult.pickedMesh;


        
        
        
        //if an object was clicked on
        if (clickedobject != null) {
            
            let clickedobjectname = clickedobject.name;
            
            //if the clicked object has a name and it isnt "plane"
            if (clickedobjectname != null){
                
                
                //if the object is already selected, and is flickable
                if (this.wasmgame.is_object_selected(clickedobjectname)){
                    
                    
                    //disable panning rotating, all camera movement basically
                    //and remporarily
                    //dont disable scrolling, it wont affect anything the player doesnt want affected when dragging
                    //this.gameappearance.camera.inputs.attached["mousewheel"].wheelPrecision = 100000;
                    this.gameappearance.camera.inputs.attached["pointers"].angularSensibilityX = 1000000;
                    this.gameappearance.camera.inputs.attached["pointers"].angularSensibilityY = 1000000;
                    
                    this.draggingobject = true;
                    
                    
                    //set the position of the cursor on the plane
                    var pickResult = this.gameappearance.scene.pick(this.gameappearance.scene.pointerX, this.gameappearance.scene.pointerY, function(mesh) {
                        return mesh.name == "plane";  // the plane will be the only pickable thing
                    });
                    
                    this.draggingstartingposition = [pickResult.pickedPoint.x, pickResult.pickedPoint.z];
                    
                    
                    
                }
                //if its not already the selected object, or is not flickable
                else{
                    
                    this.wasmgame.click_object( clickedobjectname);
                    
                }
                
                
                
            }
            //if the clicked object doesnt have a name, set the selected mesh to none
            else{
                
                this.wasmgame.click_object("");
            }
            
            
        }
        //if it wasnt, clear the selected object
        else{
            
            this.wasmgame.click_object("");
            
        }
        
        
        
    }
    
    
    
    
    
    
}