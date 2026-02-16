@echo off
cd C:\Users\simon\Documents\GitHub\RustMarket

set /p CONTRACT=Ingrese el contract address: 

REM registro a SIMON como usuario
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message registrar_usuario ^
--args "\"Simon Bierozko\"" "\"simon.bierozko@demo.com\"" Ambos ^
--value 0 ^
-y

REM registro a PEDRO como usuario
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message registrar_usuario ^
--args "\"Pedro Martinez\"" "\"pedro@demo.com\"" Ambos ^
--value 0 ^
-y

REM registro a MARIA como usuario
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message registrar_usuario ^
--args "\"Maria Hernandez\"" "\"mari@demo.com\"" Ambos ^
--value 0 ^
-y

REM registro a Carlos como usuario
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message registrar_usuario ^
--args "\"Carlos Toledo\"" "\"cacho@demo.com\"" Ambos ^
--value 0 ^
-y

REM registro a Diego como usuario
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cancel season margin scissors fault neck symptom brisk biology erupt dolphin kiss" ^
--execute ^
--message registrar_usuario ^
--args "\"Diego Flash\"" "\"diegui@demo.com\"" Ambos ^
--value 0 ^
-y

REM registro a Hernan como usuario
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "spice door reduce casino earth stomach omit bunker ability sign market square" ^
--execute ^
--message registrar_usuario ^
--args "\"Hernan Taklip\"" "\"tiktak@demo.com\"" Ambos ^
--value 0 ^
-y

REM SIMON crea categoria ALIMENTOS
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message registrar_categoria ^
--args "\"alimentos\"" ^
--value 0 ^
-y

REM SIMON crea categoria ELECTRONICA
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message registrar_categoria ^
--args "\"electronica\"" ^
--value 0 ^
-y

REM SIMON crea categoria BAZAR
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message registrar_categoria ^
--args "\"bazar\"" ^
--value 0 ^
-y

REM SIMON crea categoria DEPORTES
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message registrar_categoria ^
--args "\"deportes\"" ^
--value 0 ^
-y

REM SIMON crea categoria DECO
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message registrar_categoria ^
--args "\"deco\"" ^
--value 0 ^
-y

REM SIMON crea categoria HOGAR
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message registrar_categoria ^
--args "\"hogar\"" ^
--value 0 ^
-y

REM SIMON crea categoria INDUMENTARIA
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message registrar_categoria ^
--args "\"indumentaria\"" ^
--value 0 ^
-y

REM SIMON crea producto BANANA (1000 unidades)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_producto ^
--args "\"Banana\"" "\"Banana ecuatoriana\"" "\"alimentos\"" 1000 ^
--value 0 ^
-y

REM SIMON crea producto IPHONE X (2000 unidades)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_producto ^
--args "\"Iphone X\"" "\"Un buen celular\"" "\"electronica\"" 2000 ^
--value 0 ^
-y

REM SIMON crea producto PALITOS CHINOS (3000 unidades)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_producto ^
--args "\"Palitos Chinos\"" "\"Como un tenedor, pero peor\"" "\"bazar\"" 3000 ^
--value 0 ^
-y

REM SIMON crea producto PELOTA DE FUTBOL en categoria DEPORTES
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_producto ^
--args "\"Pelota de Futbol\"" "\"Pelota oficial FIFA\"" "\"deportes\"" 500 ^
--value 0 ^
-y

REM SIMON crea producto LAMPARA DE MESA en categoria DECO
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_producto ^
--args "\"Lampara de Mesa\"" "\"Lampara moderna LED\"" "\"deco\"" 200 ^
--value 0 ^
-y

REM SIMON crea producto SARTEN en categoria HOGAR
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_producto ^
--args "\"Sarten\"" "\"Sarten antiadherente 28cm\"" "\"hogar\"" 150 ^
--value 0 ^
-y

REM SIMON crea producto REMERA en categoria INDUMENTARIA
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_producto ^
--args "\"Remera\"" "\"Remera de algodon talle M\"" "\"indumentaria\"" 800 ^
--value 0 ^
-y

REM SIMON publica 100 BANANAS a precio 50
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_publicacion ^
--args 0 100 50 ^
--value 0 ^
-y

REM PEDRO publica 200 IPHONE X a precio 70
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message crear_publicacion ^
--args 1 200 70 ^
--value 0 ^
-y

REM MARIA publica 500 PALITOS CHINOS a precio 8
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message crear_publicacion ^
--args 2 500 8 ^
--value 0 ^
-y

REM CARLOS publica 50 PELOTAS DE FUTBOL a precio 120
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message crear_publicacion ^
--args 3 50 120 ^
--value 0 ^
-y

REM SIMON publica 25 LAMPARAS DE MESA a precio 350
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_publicacion ^
--args 4 25 350 ^
--value 0 ^
-y

REM PEDRO publica 30 SARTENES a precio 250
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message crear_publicacion ^
--args 5 30 250 ^
--value 0 ^
-y

REM MARIA publica 100 REMERAS a precio 450
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message crear_publicacion ^
--args 6 100 450 ^
--value 0 ^
-y

REM DIEGO publica 1000 REMERAS a precio 50 (id 07)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cancel season margin scissors fault neck symptom brisk biology erupt dolphin kiss" ^
--execute ^
--message crear_publicacion ^
--args 6 10 50 ^
--value 0 ^
-y

REM HERNAN publica 10 REMERAS a precio 700 (id 08)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "spice door reduce casino earth stomach omit bunker ability sign market square" ^
--execute ^
--message crear_publicacion ^
--args 6 20 700 ^
--value 0 ^
-y




REM ===== TRANSACCION 1: Pedro compra BANANAS a Simon =====
echo ORDEN 1
REM Pedro crea orden para comprar 5 BANANAS (publicacion 0)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message crear_orden ^
--args 0 5 ^
--value 0 ^
-y

REM Simon envia las BANANAS (orden 0)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message enviar_producto ^
--args 0 ^
--value 0 ^
-y

REM Pedro recibe las BANANAS
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message recibir_producto ^
--args 0 ^
--value 0 ^
-y

REM Pedro califica a Simon como vendedor (5 estrellas)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 0 5 ^
--value 0 ^
-y

REM Simon califica a Pedro como comprador (4 estrellas)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 0 4 ^
--value 0 ^
-y

REM ===== TRANSACCION 2: Simon compra IPHONE X a Pedro =====
echo ORDEN 2
REM Simon crea orden para comprar 1 IPHONE X (publicacion 1)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_orden ^
--args 1 1 ^
--value 0 ^
-y

REM Pedro envia el IPHONE X (orden 1)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message enviar_producto ^
--args 1 ^
--value 0 ^
-y

REM Simon recibe el IPHONE X
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message recibir_producto ^
--args 1 ^
--value 0 ^
-y

REM Simon califica a Pedro como vendedor (5 estrellas)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 1 5 ^
--value 0 ^
-y

REM Pedro califica a Simon como comprador (5 estrellas)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 1 5 ^
--value 0 ^
-y

REM ===== TRANSACCION 3: Carlos compra PALITOS CHINOS a Maria =====
echo ORDEN 3
REM Carlos crea orden para comprar 20 PALITOS CHINOS (publicacion 2)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message crear_orden ^
--args 2 20 ^
--value 0 ^
-y

REM Maria envia los PALITOS CHINOS (orden 2)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message enviar_producto ^
--args 2 ^
--value 0 ^
-y

REM Carlos recibe los PALITOS CHINOS
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message recibir_producto ^
--args 2 ^
--value 0 ^
-y

REM Carlos califica a Maria como vendedor (3 estrellas)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message calificar_compra ^
--args 2 3 ^
--value 0 ^
-y

REM Maria califica a Carlos como comprador (4 estrellas)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message calificar_compra ^
--args 2 4 ^
--value 0 ^
-y

REM ===== TRANSACCION 4: Pedro compra PELOTA DE FUTBOL a Carlos =====
echo ORDEN 4
REM Pedro crea orden para comprar 2 PELOTAS DE FUTBOL (publicacion 3)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message crear_orden ^
--args 3 2 ^
--value 0 ^
-y

REM Carlos envia las PELOTAS DE FUTBOL (orden 3)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message enviar_producto ^
--args 3 ^
--value 0 ^
-y

REM Pedro recibe las PELOTAS DE FUTBOL
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message recibir_producto ^
--args 3 ^
--value 0 ^
-y

REM Pedro califica a Carlos como vendedor (4 estrellas)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 3 4 ^
--value 0 ^
-y

REM Carlos califica a Pedro como comprador (5 estrellas)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message calificar_compra ^
--args 3 5 ^
--value 0 ^
-y

REM ===== TRANSACCION 5: Maria compra LAMPARA DE MESA a Simon =====
echo ORDEN 5
REM Maria crea orden para comprar 1 LAMPARA DE MESA (publicacion 4)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message crear_orden ^
--args 4 1 ^
--value 0 ^
-y

REM Simon envia la LAMPARA DE MESA (orden 4)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message enviar_producto ^
--args 4 ^
--value 0 ^
-y

REM Maria recibe la LAMPARA DE MESA
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message recibir_producto ^
--args 4 ^
--value 0 ^
-y

REM Maria califica a Simon como vendedor (5 estrellas)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message calificar_compra ^
--args 4 5 ^
--value 0 ^
-y

REM Simon califica a Maria como comprador (4 estrellas)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 4 4 ^
--value 0 ^
-y

REM ===== TRANSACCION 6: Carlos compra SARTEN a Pedro =====
echo ORDEN 6
REM Carlos crea orden para comprar 1 SARTEN (publicacion 5)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message crear_orden ^
--args 5 1 ^
--value 0 ^
-y

REM Pedro envia la SARTEN (orden 5)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message enviar_producto ^
--args 5 ^
--value 0 ^
-y

REM Carlos recibe la SARTEN
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message recibir_producto ^
--args 5 ^
--value 0 ^
-y

REM Carlos califica a Pedro como vendedor (4 estrellas)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message calificar_compra ^
--args 5 4 ^
--value 0 ^
-y

REM Pedro califica a Carlos como comprador (3 estrellas)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 5 3 ^
--value 0 ^
-y

REM ===== TRANSACCION 7: Simon compra REMERA a Maria =====
echo ORDEN 7
REM Simon crea orden para comprar 3 REMERAS (publicacion 6)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_orden ^
--args 6 3 ^
--value 0 ^
-y

REM Maria envia las REMERAS (orden 6)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message enviar_producto ^
--args 6 ^
--value 0 ^
-y

REM Simon recibe las REMERAS
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message recibir_producto ^
--args 6 ^
--value 0 ^
-y

REM Simon califica a Maria como vendedor (5 estrellas)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 6 5 ^
--value 0 ^
-y

REM Maria califica a Simon como comprador (5 estrellas)
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message calificar_compra ^
--args 6 5 ^
--value 0 ^
-y

REM ===== MOSTRAR RESULTADOS FINALES =====
REM Listar usuarios finales con sus calificaciones
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--message listar_usuarios ^
-y

REM ===== 20 TRANSACCIONES ADICIONALES =====

REM TRANSACCION 8: Carlos compra BANANAS a Simon
echo ORDEN 8
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message crear_orden ^
--args 0 8 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message enviar_producto ^
--args 7 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message recibir_producto ^
--args 7 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message calificar_compra ^
--args 7 4 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 7 5 ^
--value 0 ^
-y

REM TRANSACCION 9: Maria compra IPHONE X a Pedro
echo ORDEN 9
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message crear_orden ^
--args 1 2 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message enviar_producto ^
--args 8 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message recibir_producto ^
--args 8 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message calificar_compra ^
--args 8 5 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 8 4 ^
--value 0 ^
-y

REM TRANSACCION 10: Pedro compra PALITOS CHINOS a Maria
echo ORDEN 10
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message crear_orden ^
--args 2 15 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message enviar_producto ^
--args 9 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message recibir_producto ^
--args 9 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 9 3 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message calificar_compra ^
--args 9 4 ^
--value 0 ^
-y

REM TRANSACCION 11: Simon compra PELOTAS DE FUTBOL a Carlos
echo ORDEN 11
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_orden ^
--args 3 3 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message enviar_producto ^
--args 10 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message recibir_producto ^
--args 10 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 10 4 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message calificar_compra ^
--args 10 5 ^
--value 0 ^
-y

REM TRANSACCION 12: Carlos compra REMERAS a Maria
echo ORDEN 12
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message crear_orden ^
--args 6 5 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message enviar_producto ^
--args 11 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message recibir_producto ^
--args 11 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message calificar_compra ^
--args 11 5 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message calificar_compra ^
--args 11 3 ^
--value 0 ^
-y

REM TRANSACCION 13: Pedro compra BANANAS a Simon
echo ORDEN 13
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message crear_orden ^
--args 0 12 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message enviar_producto ^
--args 12 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message recibir_producto ^
--args 12 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 12 5 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 12 4 ^
--value 0 ^
-y

REM TRANSACCION 14: Maria compra SARTENES a Pedro
echo ORDEN 14
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message crear_orden ^
--args 5 2 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message enviar_producto ^
--args 13 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message recibir_producto ^
--args 13 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message calificar_compra ^
--args 13 4 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 13 5 ^
--value 0 ^
-y

REM TRANSACCION 15: Carlos compra LAMPARAS a Simon
echo ORDEN 15
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message crear_orden ^
--args 4 2 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message enviar_producto ^
--args 14 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message recibir_producto ^
--args 14 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message calificar_compra ^
--args 14 3 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 14 4 ^
--value 0 ^
-y

REM TRANSACCION 16: Simon compra PALITOS CHINOS a Maria
echo ORDEN 16
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_orden ^
--args 2 25 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message enviar_producto ^
--args 15 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message recibir_producto ^
--args 15 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 15 5 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message calificar_compra ^
--args 15 5 ^
--value 0 ^
-y

REM TRANSACCION 17: Pedro compra PELOTAS a Carlos
echo ORDEN 17
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message crear_orden ^
--args 3 4 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message enviar_producto ^
--args 16 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message recibir_producto ^
--args 16 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 16 4 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message calificar_compra ^
--args 16 4 ^
--value 0 ^
-y

REM TRANSACCION 18: Maria compra BANANAS a Simon
echo ORDEN 18
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message crear_orden ^
--args 0 6 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message enviar_producto ^
--args 17 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message recibir_producto ^
--args 17 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message calificar_compra ^
--args 17 5 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 17 5 ^
--value 0 ^
-y

REM TRANSACCION 19: Carlos compra IPHONE X a Pedro
echo ORDEN 19
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message crear_orden ^
--args 1 1 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message enviar_producto ^
--args 18 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message recibir_producto ^
--args 18 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message calificar_compra ^
--args 18 4 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 18 3 ^
--value 0 ^
-y

REM TRANSACCION 20: Pedro compra REMERAS a Maria
echo ORDEN 20
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message crear_orden ^
--args 6 4 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message enviar_producto ^
--args 19 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message recibir_producto ^
--args 19 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 19 5 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message calificar_compra ^
--args 19 4 ^
--value 0 ^
-y

REM TRANSACCION 21: Simon compra SARTENES a Pedro
echo ORDEN 21
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_orden ^
--args 5 3 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message enviar_producto ^
--args 20 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message recibir_producto ^
--args 20 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 20 4 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 20 5 ^
--value 0 ^
-y

REM TRANSACCION 22: Maria compra PELOTAS a Carlos
echo ORDEN 22
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message crear_orden ^
--args 3 1 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message enviar_producto ^
--args 21 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message recibir_producto ^
--args 21 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message calificar_compra ^
--args 21 3 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message calificar_compra ^
--args 21 4 ^
--value 0 ^
-y

REM TRANSACCION 23: Carlos compra BANANAS a Simon
echo ORDEN 23
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message crear_orden ^
--args 0 10 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message enviar_producto ^
--args 22 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message recibir_producto ^
--args 22 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message calificar_compra ^
--args 22 5 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 22 3 ^
--value 0 ^
-y

REM TRANSACCION 24: Pedro compra LAMPARAS a Simon
echo ORDEN 24
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message crear_orden ^
--args 4 1 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message enviar_producto ^
--args 23 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message recibir_producto ^
--args 23 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 23 4 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 23 5 ^
--value 0 ^
-y

REM TRANSACCION 25: Maria compra IPHONE X a Pedro
echo ORDEN 25
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message crear_orden ^
--args 1 1 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message enviar_producto ^
--args 24 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message recibir_producto ^
--args 24 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message calificar_compra ^
--args 24 5 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 24 5 ^
--value 0 ^
-y

REM TRANSACCION 26: Simon compra PALITOS CHINOS a Maria
echo ORDEN 26
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_orden ^
--args 2 30 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message enviar_producto ^
--args 25 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message recibir_producto ^
--args 25 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 25 4 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message calificar_compra ^
--args 25 4 ^
--value 0 ^
-y

REM TRANSACCION 27: Carlos compra REMERAS a Maria
echo ORDEN 27
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message crear_orden ^
--args 6 2 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message enviar_producto ^
--args 26 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message recibir_producto ^
--args 26 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message calificar_compra ^
--args 26 3 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message calificar_compra ^
--args 26 5 ^
--value 0 ^
-y

REM ===== TRANSACCIONES DE DIEGO FLASH =====

REM TRANSACCION 28: Diego compra BANANAS a Simon (como comprador)
echo ORDEN 28
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cancel season margin scissors fault neck symptom brisk biology erupt dolphin kiss" ^
--execute ^
--message crear_orden ^
--args 0 7 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message enviar_producto ^
--args 27 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cancel season margin scissors fault neck symptom brisk biology erupt dolphin kiss" ^
--execute ^
--message recibir_producto ^
--args 27 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cancel season margin scissors fault neck symptom brisk biology erupt dolphin kiss" ^
--execute ^
--message calificar_compra ^
--args 27 5 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 27 4 ^
--value 0 ^
-y

REM TRANSACCION 29: Diego compra PELOTAS a Carlos (como comprador)
echo ORDEN 29
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cancel season margin scissors fault neck symptom brisk biology erupt dolphin kiss" ^
--execute ^
--message crear_orden ^
--args 3 2 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message enviar_producto ^
--args 28 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cancel season margin scissors fault neck symptom brisk biology erupt dolphin kiss" ^
--execute ^
--message recibir_producto ^
--args 28 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cancel season margin scissors fault neck symptom brisk biology erupt dolphin kiss" ^
--execute ^
--message calificar_compra ^
--args 28 4 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message calificar_compra ^
--args 28 5 ^
--value 0 ^
-y

REM TRANSACCION 30: Pedro compra REMERA a Diego (Diego como vendedor)
echo ORDEN 30
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message crear_orden ^
--args 7 1 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cancel season margin scissors fault neck symptom brisk biology erupt dolphin kiss" ^
--execute ^
--message enviar_producto ^
--args 29 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message recibir_producto ^
--args 29 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 29 4 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cancel season margin scissors fault neck symptom brisk biology erupt dolphin kiss" ^
--execute ^
--message calificar_compra ^
--args 29 5 ^
--value 0 ^
-y

REM TRANSACCION 31: Maria compra REMERAS a Diego (Diego como vendedor)
echo ORDEN 31
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message crear_orden ^
--args 7 3 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cancel season margin scissors fault neck symptom brisk biology erupt dolphin kiss" ^
--execute ^
--message enviar_producto ^
--args 30 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message recibir_producto ^
--args 30 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "virtual jeans muscle share welcome teach topple butter explain ensure control cat" ^
--execute ^
--message calificar_compra ^
--args 30 3 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cancel season margin scissors fault neck symptom brisk biology erupt dolphin kiss" ^
--execute ^
--message calificar_compra ^
--args 30 4 ^
--value 0 ^
-y

REM ===== TRANSACCIONES DE HERNAN TAKLIP =====

REM TRANSACCION 32: Hernan compra SARTENES a Pedro (como comprador)
echo ORDEN 32
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "spice door reduce casino earth stomach omit bunker ability sign market square" ^
--execute ^
--message crear_orden ^
--args 5 2 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message enviar_producto ^
--args 31 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "spice door reduce casino earth stomach omit bunker ability sign market square" ^
--execute ^
--message recibir_producto ^
--args 31 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "spice door reduce casino earth stomach omit bunker ability sign market square" ^
--execute ^
--message calificar_compra ^
--args 31 5 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "cargo truth mechanic error wrestle noise cave awful exercise detail tide upset" ^
--execute ^
--message calificar_compra ^
--args 31 4 ^
--value 0 ^
-y

REM TRANSACCION 33: Hernan compra LAMPARAS a Simon (como comprador)
echo ORDEN 33
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "spice door reduce casino earth stomach omit bunker ability sign market square" ^
--execute ^
--message crear_orden ^
--args 4 1 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message enviar_producto ^
--args 32 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "spice door reduce casino earth stomach omit bunker ability sign market square" ^
--execute ^
--message recibir_producto ^
--args 32 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "spice door reduce casino earth stomach omit bunker ability sign market square" ^
--execute ^
--message calificar_compra ^
--args 32 4 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 32 5 ^
--value 0 ^
-y

REM TRANSACCION 34: Carlos compra REMERAS a Hernan (Hernan como vendedor)
echo ORDEN 34
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message crear_orden ^
--args 8 5 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "spice door reduce casino earth stomach omit bunker ability sign market square" ^
--execute ^
--message enviar_producto ^
--args 33 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message recibir_producto ^
--args 33 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "segment hunt roof write mixed special spell run wait ticket pen response" ^
--execute ^
--message calificar_compra ^
--args 33 3 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "spice door reduce casino earth stomach omit bunker ability sign market square" ^
--execute ^
--message calificar_compra ^
--args 33 4 ^
--value 0 ^
-y

REM TRANSACCION 35: Simon compra REMERAS a Hernan (Hernan como vendedor)
echo ORDEN 35
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message crear_orden ^
--args 8 3 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "spice door reduce casino earth stomach omit bunker ability sign market square" ^
--execute ^
--message enviar_producto ^
--args 34 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message recibir_producto ^
--args 34 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--execute ^
--message calificar_compra ^
--args 34 5 ^
--value 0 ^
-y

cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "spice door reduce casino earth stomach omit bunker ability sign market square" ^
--execute ^
--message calificar_compra ^
--args 34 4 ^
--value 0 ^
-y

REM ===== RESULTADOS FINALES CON NUEVOS USUARIOS =====
REM Listar todos los usuarios con calificaciones finales
cargo contract call ^
--url wss://smiling-evenly-crawdad.ngrok-free.app ^
--contract %CONTRACT% ^
--suri "alone twice window shove awake acid win manage subway behave used bless" ^
--message listar_usuarios ^
-y

pause
