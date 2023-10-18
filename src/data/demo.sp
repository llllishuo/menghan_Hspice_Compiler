*---------------26vp---------------
.option post = 2
.option search ='./'
.lib 'crn018_rfid_v1d1.l'tt
*-------------------------
m0 n1d n1g gnd gnd  nch l=180n w=2u m=1
v0 n1g gnd dc 1
v1 n1d gnd dc 2
.dc v0 0 2.5 0.01 sweep temp poi 9 0 15 30 45 60 75 90 105 120
.print i1(m0)
.end
