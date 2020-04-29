<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UPDATE ProviderLocationContactType</name>
   <tag></tag>
   <elementGuidId>13164dc1-cbd2-4f05-a4f0-b73bafb1da3e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;modifiedDataSourceId\&quot;: 0,\n  \&quot;providerLocationOperatingScheduleTypeId\&quot;: ${IdForUpdateDelete},\n  \&quot;eTag\&quot;: 0,\n  \&quot;providerLocationOperatingScheduleTypeName\&quot;: \&quot;Updated-${UUID.randomUUID().toString()}\&quot;,\n  \&quot;externalCode\&quot;: \&quot;updated ${UUID.randomUUID().toString()}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
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
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${HealthSystemAdmin}/ProviderLocationContactTypes/${IdForUpdateDelete}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.AccessToken</defaultValue>
      <description></description>
      <id>2e24b1bd-367f-49f8-88d2-574c90f35c65</id>
      <masked>false</masked>
      <name>AccessToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.HealthSystemAdmin</defaultValue>
      <description></description>
      <id>f3cafb6c-035f-4e4d-967d-3cf613df58f0</id>
      <masked>false</masked>
      <name>HealthSystemAdmin</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.IdForUpdateDelete</defaultValue>
      <description></description>
      <id>dec71654-827b-46dc-9f38-237a2605183c</id>
      <masked>false</masked>
      <name>IdForUpdateDelete</name>
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

assertThat(response.getResponseText()).contains('updated')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
