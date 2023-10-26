*------------1vp--------------------
.option post = 2
.option search ='./'
.lib 'crn018_rfid_v1d1.l'tt
.global vdd gnd
.subckt nand a b y 
m1 y a vdd vdd pch w=3u l=0.6u
m2 y b vdd vdd pch w=3u l=0.6u
m3 y b n1 gnd nch  w=2u l=0.6u
m4 n1 a gnd gnd nch w=2u l=0.6u
.ends 

.subckt nor a b y
m1 n1 a vdd vdd pch w=3u l=0.6u
m2 y b n1 vdd pch   w=3u l=0.6u
m3 y b n1 gnd nch   w=2u l=0.6u
m4 y a gnd gnd nch  w=2u l=0.6u
.ends 


.subckt inv1x a y
m1 y a vdd vdd pch w=3u l=0.6u
m2 y a gnd gnd nch w=1.5u l=0.6u 
.ends 


v1 vdd gnd 2 
v2 a gnd pulse 0 2 10n 0.2n 0.2n 10n 20n
v3 b gnd pulse 0 2 10n 0.2n 0.2n 20n 40n
X1 a b y nand 
c1 y gnd 0.1p 
.tran 0.1n 80n 
.end
