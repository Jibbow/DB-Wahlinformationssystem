@ECHO Starting
call ./jmeter -n -t Aggn10t800.jmx -l n10t800.log -e -o on10t800
@ECHO First finished
call ./jmeter -n -t Aggn10t1200.jmx -l n10t1200.log -e -o on10t1200
@ECHO Finished n = 10, t = 1200
call ./jmeter -n -t Aggn50t800.jmx -l n50t800.log -e -o on50t800
@ECHO Finished n = 50, t = 800
call ./jmeter -n -t Aggn50t1200.jmx -l n50t1200.log -e -o on50t1200
@ECHO Finished n = 50, t = 1200
call ./jmeter -n -t Aggn100t800.jmx -l n100t800.log -e -o on100t800
@ECHO Finished n = 100, t = 800
call ./jmeter -n -t Aggn100t800.jmx -l n100t1200.log -e -o on100t1200
@ECHO Finished n = 100, t = 1200
call ./jmeter -n -t Aggn300t800.jmx -l n300t800.log -e -o on300t800
@ECHO Finished n = 300, t = 800
call ./jmeter -n -t Aggn300t1200.jmx -l n300t1200.log -e -o on300t1200
@ECHO Finished n = 300, t = 1200
call ./jmeter -n -t Aggn1000t800.jmx -l n1000t800.log -e -o on1000t800
@ECHO Finished n = 10, t = 1200
call ./jmeter -n -t Aggn1000t1200.jmx -l n1000t1200.log -e -o on1000t1200
