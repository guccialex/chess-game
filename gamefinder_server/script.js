

//http://0.0.0.0:8082/index.html?port=2432&password=F38d



//connect to the matchmaker websocket server
let websocketaddress = 'ws://127.0.0.1:3050';


let socket = new WebSocket( websocketaddress );
    
    
socket.onopen = function (event) {
    
    console.log("connected to the matchmaking server");
};


socket.onmessage = function (event){


    let receiveddata = JSON.parse(event.data);

    if (receiveddata.gameport != null && receiveddata.gamepassword != null){


        
        let gamedataserver = "http://0.0.0.0:8000/index.html";

        let portinfo = "?port=" + receiveddata.gameport;

        let passwordinfo = "&password=" + receiveddata.gamepassword;


        window.location.href = gamedataserver + portinfo + passwordinfo;
    }



}




function ConnectToPublicGame() {
    document.getElementById("demo").innerHTML = "connecting to public game";

    socket.send(  JSON.stringify("joinpublicgame")  );
}


function ConnectToPrivateGame() {
    document.getElementById("demo").innerHTML = "connecting to private game";

    let password = document.getElementById("gamepassword").value;

    let tosend =  JSON.stringify({joinprivategame: password})  ;

    socket.send(tosend);
}


function CreatePrivateGame() {
    document.getElementById("demo").innerHTML = "creating private game";

    socket.send(JSON.stringify("createprivategame"));
}



