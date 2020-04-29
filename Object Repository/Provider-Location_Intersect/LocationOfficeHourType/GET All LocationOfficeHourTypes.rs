<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Get All ProviderLocationContactTypes</description>
   <name>GET All LocationOfficeHourTypes</name>
   <tag></tag>
   <elementGuidId>74771f5d-e96f-44a5-a7f9-c633a9d38d6f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${AccessToken}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${HealthSystemAdmin}/ProviderLocationOperatingScheduleTypes</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.AccessToken</defaultValue>
      <description></description>
      <id>7c19391a-a0b9-4eda-866d-cd6d1802cabc</id>
      <masked>false</masked>
      <name>AccessToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.HealthSystemAdmin</defaultValue>
      <description></description>
      <id>fecb97aa-58a6-4341-ac53-68f6e1b35afe</id>
      <masked>false</masked>
      <name>HealthSystemAdmin</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
